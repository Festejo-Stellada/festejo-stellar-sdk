use pyo3::prelude::*;
use festejo_frsp::crypto;

#[pyfunction]
fn sign_transaction(tx: Vec<u8>, secret: String) -> PyResult<String> {
    Ok(crypto::sign_data(&tx, &secret))
}

#[pymodule]
fn festejo_rust_stellar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sign_transaction, m)?)?;
    Ok(())
}
