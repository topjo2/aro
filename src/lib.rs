use pyo3::{prelude::*, types::PyModule};

/// Fonction de multiplication
#[pyfunction]
fn multiplier(a: i32, b: i32) -> i32 {
    a * b
}

/// Message d'accueil
#[pyfunction]
fn salutation() -> &'static str {
    "Bonjour, je suis Aro, votre assistante Rust! 💖"
}

/// Message de départ
#[pyfunction]  // Cette annotation est cruciale
fn au_revoir() -> &'static str {
    "Au revoir, merci d'avoir utilisé Aro! 💫"
}

/// Module Python
#[pymodule]
fn aro(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiplier, py)?)?;
    m.add_function(wrap_pyfunction!(salutation, py)?)?;
    m.add_function(wrap_pyfunction!(au_revoir, py)?)?;  // Ne pas oublier cette ligne
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(2, 3), 6);
    }
}
