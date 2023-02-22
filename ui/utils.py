import sqlite3
from socket import socket
from subprocess import Popen, check_call
from time import sleep

from flask import g

DATABASE = "db.sqlite3"


def query_db(query, args=(), one=False):
    cur = get_db().execute(query, args)
    rv = cur.fetchall()
    cur.close()
    return (rv[0] if rv else None) if one else rv


def get_db():
    db = getattr(g, "_database", None)
    if db is None:
        db = g._database = sqlite3.connect(DATABASE, isolation_level=None)
        db.row_factory = sqlite3.Row
    return db


def init_db():
    from app import app

    with app.app_context():
        db = get_db()
        with app.open_resource("interlockings.sql", mode="r") as f:
            db.cursor().executescript(f.read())
        db.commit()


def generate_interlocking(rowid):
    try:
        # find free port
        with socket() as s:
            s.bind(("", 0))
            port = s.getsockname()[1]

        # generate code
        check_call(
            [
                "cargo",
                "run",
                "--package",
                "code_generation",
                "--",
                "-o",
                f"ixl_{rowid}",
                f"ui/generated/{rowid}_routes.json",
                "grpc",
                "--addr",
                f"0.0.0.0:{port}",
                "--topology",
                f"ui/generated/{rowid}_topology.json",
                "--placement",
                f"ui/generated/{rowid}_placement.json",
            ],
            cwd=r"../",
        )
        check_call(["cargo", "build"], cwd=f"../ixl_{rowid}")

        # start interlocking
        process = Popen(["cargo", "run"], cwd=f"../ixl_{rowid}")

        # wait for interlocking to be online
        sleep(1)

        # write to database
        from app import app

        with app.app_context():
            query_db(
                f"UPDATE interlockings SET port = {port}, state=1 WHERE ROWID={rowid}"
            )

        process.wait()

        with app.app_context():
            query_db(f"UPDATE interlockings SET state=2 WHERE ROWID={rowid}")

    except Exception as e:
        print(e)
        from app import app

        with app.app_context():
            query_db(f"UPDATE interlockings SET state=3 WHERE ROWID={rowid}")
