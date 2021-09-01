use super::structures::ExecuteOutput;
use crate::parse::structures::Numeric;
use std::collections::HashMap;

pub fn execute_access(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_access_array_with_array(lhs_array, rhs_array),
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::String (rhs_string)) => execute_access_array_with_string(lhs_array, rhs_string),
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Numeric (rhs_numeric)) => execute_access_array_with_numeric(lhs_array, rhs_numeric),
        (ExecuteOutput::Map (lhs_dict), ExecuteOutput::String (rhs_string)) => execute_access_dict_with_string(lhs_dict, rhs_string),
        (lhs_other, rhs_other) => panic!("Cannot use access with {:?} . {:?}", lhs_other, rhs_other)
    }
}

fn execute_access_array_with_array(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    let mut indicies: Vec<usize> = Vec::new();

    for val in rhs_array {
        match val {
            ExecuteOutput::Numeric (Numeric::Int(x)) => indicies.push(x as usize),
            other => panic!("Cannot use non int to access array {:?}", other)
        }
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    for index in indicies {
        output.push(lhs_array.get(index).unwrap().clone())
    }

    ExecuteOutput::Array(output)
}

fn execute_access_array_with_string(lhs_array: Vec<ExecuteOutput>, rhs_string: String) -> ExecuteOutput {
    let mut dicts: Vec<HashMap<String, ExecuteOutput>> = Vec::new();

    for val in lhs_array {
        match val {
            ExecuteOutput::Map (dict) => dicts.push(dict),
            other => panic!("Cannot access array with string, if not array of dicts. Found {:?}", other)
        }
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    // wrap up string and dicts into ExecuteOutput in order to pass back into execute_access
    let rhs_string = ExecuteOutput::String(rhs_string);
    for dict in dicts {
        let dict = ExecuteOutput::Map(dict);
        output.push(execute_access(dict, rhs_string.clone()));
    }

    ExecuteOutput::Array(output)
}

fn execute_access_array_with_numeric(lhs_array: Vec<ExecuteOutput>, rhs_numeric: Numeric) -> ExecuteOutput {
    let rhs_numeric: usize = match rhs_numeric {
        Numeric::Int (x) => x as usize,
        other => panic!("Cannot access array via non int {:?} numeric", other)
    };

    lhs_array.get(rhs_numeric).unwrap().clone()
}

fn execute_access_dict_with_string(lhs_dict: HashMap<String, ExecuteOutput>, rhs_string: String) -> ExecuteOutput {
    println!("Accessing dict {:?} with {:?}", lhs_dict, rhs_string);
    lhs_dict.get(&rhs_string).unwrap().clone()
}