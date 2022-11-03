use pyo3::prelude::*;

#[pyclass]
struct PySignal {
    uuid: String,
    previous_node: PyNode,
    next_node: PyNode,
    function: String,
    distance: f32,
    wirkrichtung: String,
    name: String
}

#[pyclass]
struct PyNode {
    uuid: String,
    connected_on_head: Option<Box<PyNode>>,
    connected_on_left: Option<Box<PyNode>>,
    connected_on_right: Option<Box<PyNode>>,
    connected_nodes: Vec<Box<PyNode>>,
}

#[pyclass]
struct PyEdge {
    uuid: String,
    node_a: PyNode,
    node_b: PyNode,
    signals: Vec<PySignal>,
    length: f32
}

#[pyclass]
struct PyRoute {
    uuid: String,
    vmax: u32
}
