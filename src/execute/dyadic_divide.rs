use super::structures::ExecuteOutput;
use crate::parse::structures::Numeric;
use std::collections::HashMap;

pub fn execute_divide(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        // Divide an array by an array
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_divide_array_by_array(lhs_array, rhs_array),
        // Divide a dictionary by a number
        (ExecuteOutput::Map (lhs_map), ExecuteOutput::Numeric (rhs_numeric)) => execute_divide_dict_by_numeric(lhs_map, rhs_numeric),
        // Divide an array by a number
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Numeric (numeric)) => execute_divide_array_by_numeric(lhs_array, numeric),
        // Divide a number by a number
        (ExecuteOutput::Numeric (lhs_numeric), ExecuteOutput::Numeric (rhs_numeric)) => execute_divide_numeric_by_numeric(lhs_numeric, rhs_numeric),
        (lhs_other, rhs_other) => panic!("Cannot divide pair ({:?}, {:?})", lhs_other, rhs_other)
    }
}

fn execute_divide_array_by_numeric(lhs_array: Vec<ExecuteOutput>, numeric: Numeric) -> ExecuteOutput {
    let mut output: Vec<ExecuteOutput> = Vec::new();

    // Wrap numeric in an ExecuteOutput so it can be passed back into calculate_divide
    let numeric = ExecuteOutput::Numeric(numeric);
    for val in lhs_array {
        output.push(execute_divide(val, numeric.clone()));
    }

    ExecuteOutput::Array(output)
}

fn execute_divide_array_by_array(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot divide two arrays of different size {:?} vs {:?}", lhs_array, rhs_array);
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    for i in 0..lhs_array.len() {
        let lhs_value = lhs_array.get(i).unwrap().clone();

        let rhs_value = rhs_array.get(i).unwrap().clone();

        output.push(execute_divide(lhs_value, rhs_value));
    }

    ExecuteOutput::Array(output)
}

fn execute_divide_dict_by_numeric(lhs_map: HashMap<String, ExecuteOutput>, numeric: Numeric) -> ExecuteOutput {
    let mut output: HashMap<String, ExecuteOutput> = HashMap::new();

    // Wrap numeric in an ExecuteOutput so it can be passed back into calculate_divide
    let numeric = ExecuteOutput::Numeric(numeric);
    for (key, val) in lhs_map {
        output.insert(key, execute_divide(val, numeric.clone()));
    }

    ExecuteOutput::Map(output)
}

fn execute_divide_numeric_by_numeric(lhs_numeric: Numeric, rhs_numeric: Numeric) -> ExecuteOutput {
    let lhs_float = match lhs_numeric {
        Numeric::Float(x) => x,
        Numeric::Int(x) => x as f64
    };

    let rhs_float = match rhs_numeric {
        Numeric::Float(x) => x,
        Numeric::Int(x) => x as f64
    };

    let result: f64 = lhs_float / rhs_float;

    ExecuteOutput::Numeric(Numeric::Float(result))
}