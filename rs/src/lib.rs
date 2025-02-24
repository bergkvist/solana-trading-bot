#[pyo3::pymodule]
mod rs {
    #[pyo3::pyfunction]
    fn add(x: u64, y: u64) -> u64 {
        x + y
    }
}
