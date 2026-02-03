use pyo3::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[pyfunction]
fn download_json(url: String) -> PyResult<String> {
    let response = reqwest::blocking::get(&url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let text = response.text()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    Ok(text)
}

#[pyfunction]
fn download_file(url: String) -> PyResult<String> {
    let mut path = PathBuf::from(std::env::var("USERPROFILE").unwrap());
    path.push("Downloads");
    path.push("update_file.bin");

    let mut resp = reqwest::blocking::get(&url)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let mut out = File::create(&path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    let bytes = resp.bytes()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    out.write_all(&bytes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e.to_string()))?;

    Ok(path.to_string_lossy().to_string())
}

#[pymodule]
fn rust_updater(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(download_json, m)?)?;
    m.add_function(wrap_pyfunction!(download_file, m)?)?;
    Ok(())
}
