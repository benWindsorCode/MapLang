use crate::parse::structures::Numeric;
use super::structures::ExecuteOutput;
use std::collections::HashMap;

// TODO: ideally it should be that you can always compare to an item at least one 'rank' less than the current
// e.g. can do [ [...], [...], [...] ] = [...], or [ [...], [...] ] = 5 or [ 1, 2, 3 ] = 5
pub fn execute_equals(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_equals_arrays(lhs_array, rhs_array),
        (ExecuteOutput::Array (array), ExecuteOutput::Numeric (numeric) )
            | ( ExecuteOutput::Numeric (numeric), ExecuteOutput::Array (array) ) => execute_equals_array_and_numeric(array, numeric),
        (ExecuteOutput::Map (map), ExecuteOutput::Numeric (numeric)) 
            | (ExecuteOutput::Numeric (numeric), ExecuteOutput::Map (map)) => execute_equals_map_and_numeric(map, numeric),
        (ExecuteOutput::Numeric (lhs_numeric), ExecuteOutput::Numeric (rhs_numeric)) => execute_equals_numerics(lhs_numeric, rhs_numeric),
        (lhs_other, rhs_other) => panic!("Cannot perform equals on {:?} = {:?}", lhs_other, rhs_other)
    }
}

fn execute_equals_arrays(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot equate arrays of different lengths {:?} vs {:?}", lhs_array.len(), rhs_array.len());
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    for i in 0..lhs_array.len() {
        let lhs_val = lhs_array.get(i).unwrap().clone();
        let rhs_val = rhs_array.get(i).unwrap().clone();

        output.push(execute_equals(lhs_val, rhs_val));
    }

    ExecuteOutput::Array(output)
}

fn execute_equals_map_and_numeric(map: HashMap<String, ExecuteOutput>, numeric: Numeric) -> ExecuteOutput {
    let mut output: HashMap<String, ExecuteOutput> = HashMap::new();

    let numeric = ExecuteOutput::Numeric(numeric);
    for (key, value) in map {
        output.insert(key, execute_equals(value, numeric.clone()));
    }

    ExecuteOutput::Map(output)
}

fn execute_equals_array_and_numeric(array: Vec<ExecuteOutput>, numeric: Numeric) -> ExecuteOutput {
    let mut output: Vec<ExecuteOutput> = Vec::new();

    let numeric = ExecuteOutput::Numeric(numeric);
    for val in array {
        output.push(execute_equals(val, numeric.clone()));
    }

    ExecuteOutput::Array(output)
}

fn execute_equals_numerics(lhs_numeric: Numeric, rhs_numeric: Numeric) -> ExecuteOutput {
    // TODO: create a bool type
    let equal = match lhs_numeric == rhs_numeric {
        true => Numeric::Int(1),
        false => Numeric::Int(0)
    };

    ExecuteOutput::Numeric(equal)
}