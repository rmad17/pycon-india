use pyo3::prelude::PyErr;
use pyo3::prelude::PyResult;
use pyo3::prelude::*;
use pyo3::pyfunction;
use pyo3::pymodule;
use pyo3::types::PyList;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::Bound;

use std::io;

// Formats the sum of two numbers as string.
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

#[pyfunction]
fn sum_as_string(filename: String) -> PyResult {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut records: Vec<String> = vec![String::new(); 126];
    for (index, result) in rdr.records().enumerate() {
        // for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result;
        println!("{:?}", record);
        records.insert(index, result);
    }
    Ok(records);
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pyrust_csv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
