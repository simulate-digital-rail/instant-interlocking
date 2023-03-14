import json
import threading
from urllib.parse import urlparse

import requests
from flask import Flask, request, render_template, url_for, g, redirect

from interlocking_exporter.exporter import Exporter as InterlockingExporter
from orm_importer.importer import ORMImporter

from utils import query_db, generate_interlocking

import logging
log = logging.getLogger('werkzeug')
log.setLevel(logging.WARNING)

app = Flask(__name__)


@app.route("/")
def homepage():
    running_ixls = query_db("SELECT rowid, * FROM interlockings WHERE state=1")
    return render_template(
        "index.html",
        css_file=url_for("static", filename="pico.min.css"),
        axios_file=url_for("static", filename="axios.min.js"),
        modal_file=url_for("static", filename="modal.js"),
        running_ixls=running_ixls,
        hostname=urlparse(request.base_url).hostname,
        base_url=request.base_url,
    )


@app.route("/run")
def run_converter():
    # generate topology from polygon
    polygon = request.args.get("polygon")
    if not polygon:
        return "No location specified", 400
    
    try:
        topology = ORMImporter().run(polygon)
    except Exception as e:
        return f"Exception while generating topology: `{e}`.\nThis probably means you need to tweak your selected polygon.", 500

    # persist to database
    query_db("INSERT INTO interlockings (title) VALUES ('test');")
    rowid = query_db("SELECT last_insert_rowid()", one=True)["last_insert_rowid()"]

    # export JSON files for interlocking
    exporter = InterlockingExporter(topology)
    try:
        with open(f"generated/{rowid}_routes.json", "w") as routes_file:
            json.dump(exporter.export_routes(), routes_file)
        with open(f"generated/{rowid}_topology.json", "w") as topology_file:
            json.dump(exporter.export_topology(), topology_file)
        with open(f"generated/{rowid}_placement.json", "w") as placement_file:
            json.dump(exporter.export_placement(), placement_file)
    except Exception as e:
        return f"Exception while generating JSON files: `{e}`.\nThis probably means you need to tweak your selected polygon.", 500

    # start build process
    thread = threading.Thread(target=generate_interlocking, args=(rowid,))
    thread.start()

    return json.dumps({"id": rowid}), 200


@app.route("/status/<rowid>")
def get_status(rowid=0):
    # get from database
    result = query_db(f"SELECT * FROM interlockings WHERE ROWID={rowid}", one=True)
    match result["state"]:
        case 0:
            return json.dumps({"id": rowid, "state": "generating"}), 200
        case 1:
            return (
                json.dumps({"id": rowid, "state": "running", "port": result["port"]}),
                200,
            )
        case 2:
            return json.dumps({"id": rowid, "state": "stopped"}), 200
        case 3:
            return json.dumps({"id": rowid, "state": "failed"}), 200


@app.route("/terminate/<rowid>")
def terminate(rowid=0):
    result = query_db(f"SELECT * FROM interlockings WHERE ROWID={rowid}", one=True)
    try:
        response = requests.get(
            f"http://{urlparse(request.base_url).hostname}:{result['port']}/terminate"
        )
        if response.status_code != 200:
            return response.text, response.status_code
        query_db(f"UPDATE interlockings SET state=2 WHERE ROWID={rowid}")
    except requests.exceptions.ConnectionError:
        query_db(f"UPDATE interlockings SET state=3 WHERE ROWID={rowid}")
    return redirect("/")


@app.teardown_appcontext
def close_connection(exception):
    db = getattr(g, "_database", None)
    if db is not None:
        db.close()


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0")
