use pyo3::prelude::*;

#[pyfunction]
fn count_char(string: String, target: char) -> PyResult<usize> {
    Ok(string.chars().filter(|&c| c == target).count())
}

#[pyfunction]
fn mine_advent_coin(string: String, start: String) -> PyResult<usize> {
    let mut value: usize = 0;

    loop {
        let mut input = string.clone();
        input.push_str(&value.to_string());
        let digest = format!("{:x}", md5::compute(input.as_bytes()));
        if digest.starts_with(&start) {
            return Ok(value);
        }
        value += 1;
    }
}

#[pymodule]
fn aoc_py03(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_char, m)?)?;
    m.add_function(wrap_pyfunction!(mine_advent_coin, m)?)?;
    Ok(())
}
