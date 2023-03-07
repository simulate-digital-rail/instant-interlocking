# Interlocking Generator UI

This web app allows you to generate an interlocking by selecting an area in OpenRailwayMap.

## How to run it

- Install Python and [Poetry](https://python-poetry.org/)
- Create a new venv (`python -m venv venv`) and activate it
- Run `poetry install` to install dependencies
- Run the app with `python app.py` and navigate to http://localhost:5000 in your browser

## Caveats

It can be difficult to find the right bounding box for a station. If you get internal server errors, try
moving your selection around a bit. Also keep in mind that OpenRailwayMap is open source and maintained
by volunteers, so the data can sometimes be incomplete or incorrect.