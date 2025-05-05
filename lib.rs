// Supprimez ou commentez proprement les lignes problématiques comme:
// # 2. Builder

// Assurez-vous que votre code Rust est valide, par exemple:
use pyo3::prelude::*;

/// Fonction exemple
#[pyfunction]
fn hello() -> String {
    "Bonjour de Rust!".to_string()
}

/// Module Python
#[pymodule]
fn my_project(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
