use super::structures::ExecuteOutput;
use std::collections::HashMap;
use crate::parse::structures::Numeric;

pub fn execute_add(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        // Adding two arrays of numbers
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_add_arrays(lhs_array, rhs_array),
        // Adding an array + number
        (ExecuteOutput::Array (int_array), ExecuteOutput::Numeric (numeric_val))
            | (ExecuteOutput::Numeric (numeric_val), ExecuteOutput::Array(int_array)) => execute_add_array_and_numeric(int_array, numeric_val),
        // Adding two numbers
        (ExecuteOutput::Numeric (lhs_val), ExecuteOutput::Numeric (rhs_val)) => ExecuteOutput::Numeric(lhs_val + rhs_val),
        // Adding two dicts
        (ExecuteOutput::Map (lhs_dict), ExecuteOutput::Map (rhs_dict)) => execute_add_dicts(lhs_dict, rhs_dict),
        (lhs_other, rhs_other) => panic!("Cannot add pair ({:?}, {:?})", lhs_other, rhs_other)
    }
}

fn execute_add_arrays(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot add arrays of different lengths {:?} vs {:?}", lhs_array.len(), rhs_array.len());
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();
    
    for i in 0..lhs_array.len() {
        let lhs_val = lhs_array.get(i).unwrap().clone();
        let rhs_val = rhs_array.get(i).unwrap().clone();

        output.push(execute_add(lhs_val, rhs_val));
    }

    ExecuteOutput::Array(output)
}

fn execute_add_dicts(lhs_dict: HashMap<String, ExecuteOutput>, rhs_dict: HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    if lhs_dict.len() != rhs_dict.len() {
        panic!("Cannot add dicts of different lengths: {:?} vs {:?}", lhs_dict.len(), rhs_dict.len());
    }

    let mut output: HashMap<String, ExecuteOutput> = HashMap::new();

    for (key, value) in lhs_dict {
        let rhs_value = rhs_dict.get(&key).unwrap().clone();
        output.insert(key, execute_add(value, rhs_value));
    }

    ExecuteOutput::Map(output)
}

fn execute_add_array_and_numeric(int_array:  Vec<ExecuteOutput>, int_val: Numeric) -> ExecuteOutput {
    // re-wrap numeric in an ExecuteOutput to allow being passed back into execute_add
    let int_val = ExecuteOutput::Numeric(int_val);

    let output = int_array.into_iter().map(|x| execute_add(x, int_val.clone())).collect();

    ExecuteOutput::Array(output)
}