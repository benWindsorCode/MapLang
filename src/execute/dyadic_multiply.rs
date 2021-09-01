
use super::structures::ExecuteOutput;
use std::collections::HashMap;
use crate::parse::structures::Numeric;

pub fn execute_multiply(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        // Multiply two arrays of numbers
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_multiply_arrays(lhs_array, rhs_array),
        // Multiply an array + number
        (ExecuteOutput::Array (int_array), ExecuteOutput::Numeric (numeric_val))
            | (ExecuteOutput::Numeric (numeric_val), ExecuteOutput::Array(int_array)) => execute_multiply_array_and_numeric(int_array, numeric_val),
        // Multiply two numbers
        (ExecuteOutput::Numeric (lhs_val), ExecuteOutput::Numeric (rhs_val)) => ExecuteOutput::Numeric(lhs_val * rhs_val),
        // Multiply two dicts
        (ExecuteOutput::Map (lhs_map), ExecuteOutput::Map (rhs_map)) => execute_multiply_dicts(lhs_map, rhs_map),
        (lhs_other, rhs_other) => panic!("Cannot multiply pair ({:?}, {:?})", lhs_other, rhs_other)
    }
}

fn execute_multiply_arrays(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot multiply arrays of different lengths {:?} vs {:?}", lhs_array.len(), rhs_array.len());
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();
    
    for i in 0..lhs_array.len() {
        let lhs_val = lhs_array.get(i).unwrap().clone();
        let rhs_val = rhs_array.get(i).unwrap().clone();

        output.push(execute_multiply(lhs_val, rhs_val));
    }

    ExecuteOutput::Array(output)
}

fn execute_multiply_dicts(lhs_map: HashMap<String, ExecuteOutput>, rhs_map: HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    if lhs_map.len() != rhs_map.len() {
        panic!("Cannot multiply dicts of different lengths: {:?} vs {:?}", lhs_map.len(), rhs_map.len());
    }

    let mut output: HashMap<String, ExecuteOutput> = HashMap::new();

    for (key, value) in lhs_map {
        let rhs_value = rhs_map.get(&key).unwrap().clone();
        output.insert(key, execute_multiply(value, rhs_value));
    }

    ExecuteOutput::Map(output)
}

fn execute_multiply_array_and_numeric(int_array:  Vec<ExecuteOutput>, int_val: Numeric) -> ExecuteOutput {
    // re-wrap numeric in an ExecuteOutput to allow being passed back into execute_multiply
    let int_val = ExecuteOutput::Numeric(int_val);

    let output = int_array.into_iter().map(|x| execute_multiply(x, int_val.clone())).collect();

    ExecuteOutput::Array(output)
}