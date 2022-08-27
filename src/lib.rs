use pyo3::prelude::*;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

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

#[pyfunction]
fn count_on_lights(instructions: Vec<((u16, u16, u16, u16), char)>) -> PyResult<usize> {
    let mut lights: HashSet<(u16, u16)> = HashSet::new();

    for instruction in instructions {
        for i in instruction.0 .0..=instruction.0 .2 {
            for j in instruction.0 .1..=instruction.0 .3 {
                match instruction.1 {
                    't' => {
                        let tuple = (i, j);
                        if lights.contains(&tuple) {
                            lights.remove(&tuple);
                        } else {
                            lights.insert(tuple);
                        }
                        true
                    }
                    'n' => lights.insert((i, j)),
                    'f' => lights.remove(&(i, j)),
                    _ => true,
                };
            }
        }
    }

    Ok(lights.len())
}

#[pyfunction]
fn count_total_brightness(instructions: Vec<((u16, u16, u16, u16), char)>) -> PyResult<isize> {
    let mut lights: HashMap<(u16, u16), isize> = HashMap::new();

    for instruction in instructions {
        for i in instruction.0 .0..=instruction.0 .2 {
            for j in instruction.0 .1..=instruction.0 .3 {
                let tuple = (i, j);
                if !lights.contains_key(&tuple) {
                    lights.insert(tuple, 0);
                }
                let value: isize = match instruction.1 {
                    't' => *lights.get(&tuple).unwrap() + 2,
                    'n' => *lights.get(&tuple).unwrap() + 1,
                    'f' => max(*lights.get(&tuple).unwrap() - 1, 0),
                    _ => 0,
                };
                lights.insert(tuple, value);
            }
        }
    }

    Ok(lights.values().sum::<isize>())
}

#[pymodule]
fn aoc_py03(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_char, m)?)?;
    m.add_function(wrap_pyfunction!(count_on_lights, m)?)?;
    m.add_function(wrap_pyfunction!(count_total_brightness, m)?)?;
    m.add_function(wrap_pyfunction!(mine_advent_coin, m)?)?;
    Ok(())
}
