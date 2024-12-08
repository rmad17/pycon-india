use pyo3::prelude::PyResult;
use pyo3::prelude::*;
use pyo3::pyfunction;
use pyo3::pymodule;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::Bound;
use std::error::Error;

// Formats the sum of two numbers as string.
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

#[pyfunction]
fn csv_records(filename: String) -> PyResult<()> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut records: Vec<String> = vec![String::new(); 126];
    for (index, result) in rdr.records().enumerate() {
        let record = result;
        println!("{:?}", record);
        records.insert(index, result);
    }
    Ok(records)
    //for result in rdr.deserialize() {
    //    // Notice that we need to provide a type hint for automatic
    //    // deserialization.
    //    let record = result?;
    //    println!("{:?}", record);
    //}
    //Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pyrust_csv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(csv_records, m)?)?;
    Ok(())
}
