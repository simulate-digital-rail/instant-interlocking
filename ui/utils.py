import sqlite3
from socket import socket
from subprocess import Popen
from time import sleep

from flask import g

DATABASE = "db.sqlite3"


def query_db(query, args=(), one=False):
    cur = get_db().execute(query, args)
    rv = cur.fetchall()
    cur.close()
    return (rv[0] if rv else None) if one else rv


def get_db():
    db = getattr(g, '_database', None)
    if db is None:
        db = g._database = sqlite3.connect(DATABASE, isolation_level=None)
        db.row_factory = sqlite3.Row
    return db


def init_db():
    from ui.app import app
    with app.app_context():
        db = get_db()
        with app.open_resource('interlockings.sql', mode='r') as f:
            db.cursor().executescript(f.read())
        db.commit()


def generate_interlocking(rowid, filename="driveways.json"):
    # generate code
    print("generating code")

    # find free port
    with socket() as s:
        s.bind(('',0))
        port = s.getsockname()[1]

    # start process
    Popen(["cargo", "run", "--example", "dev_ixl"], cwd=r"../code_generation")

    # wait for interlocking to be online
    sleep(1)

    # write to database
    from ui.app import app
    with app.app_context():
        query_db(f"UPDATE interlockings SET port = {port}, state=1 WHERE ROWID={rowid}")
