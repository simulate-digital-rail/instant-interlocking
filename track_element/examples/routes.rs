use std::fs;

use pyo3::prelude::*;
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let py_app = include_str!("d:\\Daten\\workspace\\Masterprojekt\\codegen\\dummy_driveway_generator\\src\\main.py");

    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("write_data")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);

    let data = fs::read_to_string("data.json")?;
    let v: Value = serde_json::from_str(&data)?;

    println!("{}", v["routes"][0]);
    Ok(())
}
