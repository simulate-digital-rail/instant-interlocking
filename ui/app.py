import json
import threading
from urllib.parse import urlparse

from flask import Flask, request, render_template, url_for, g

from driveway_generator.main import generate_driveway_json
from ui.utils import query_db, generate_interlocking

app = Flask(__name__)


@app.route("/")
def homepage():
    running_ixls = query_db("SELECT rowid, * FROM interlockings WHERE state=1")
    return render_template('index.html', css_file=url_for('static', filename='pico.min.css'), axios_file=url_for('static', filename='axios.min.js'), modal_file=url_for('static', filename='modal.js'), running_ixls=running_ixls, hostname=urlparse(request.base_url).hostname)


@app.route("/run")
def run_converter():
    # generate driveway JSON from polygon
    polygon = request.args.get('polygon')
    if not polygon:
        return 'No location specified', 400
    generate_driveway_json(polygon)

    # persist to database
    query_db("INSERT INTO interlockings (title) VALUES ('test');")
    rowid = query_db("SELECT last_insert_rowid()", one=True)["last_insert_rowid()"]

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
            return json.dumps({"id": rowid, "state": "running", "port": result["port"]}), 200
        case 2:
            return json.dumps({"id": rowid, "state": "stopped"}), 200
        case 3:
            return json.dumps({"id": rowid, "state": "failed"}), 200


@app.teardown_appcontext
def close_connection(exception):
    db = getattr(g, '_database', None)
    if db is not None:
        db.close()


if __name__ == "__main__":
    app.run(debug=True)
