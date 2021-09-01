use crate::parse::structures::Numeric;
use super::structures::ExecuteOutput;

pub fn execute_equals(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_equals_arrays(lhs_array, rhs_array),
        (ExecuteOutput::Array (array), ExecuteOutput::Numeric (numeric) )
            | ( ExecuteOutput::Numeric (numeric), ExecuteOutput::Array (array) ) => execute_equals_array_and_numeric(array, numeric),
        (ExecuteOutput::Numeric (lhs_numeric), ExecuteOutput::Numeric (rhs_numeric)) => execute_equals_numerics(lhs_numeric, rhs_numeric),
        (lhs_other, rhs_other) => panic!("Cannot perform equals on {:?} = {:?}", lhs_other, rhs_other)
    }
}

fn execute_equals_arrays(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    let mut lhs_numerics: Vec<Numeric> = Vec::new();
    for val in lhs_array {
        match val {
            ExecuteOutput::Numeric (x) => lhs_numerics.push(x),
            other => panic!("Cannot equate non numerics currently. {:?}", other)
        }
    }

    let mut rhs_numerics: Vec<Numeric> = Vec::new();
    for val in rhs_array {
        match val {
            ExecuteOutput::Numeric (x) => rhs_numerics.push(x),
            other => panic!("Cannot equate non numerics currently. {:?}", other)
        }
    }

    if lhs_numerics.len() != rhs_numerics.len() {
        panic!("Cannot equate arrays of different lengths {:?} vs {:?}", lhs_numerics.len(), rhs_numerics.len());
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    for i in 0..lhs_numerics.len() {
        let lhs_val = ExecuteOutput::Numeric(lhs_numerics.get(i).unwrap().clone());
        let rhs_val = ExecuteOutput::Numeric(rhs_numerics.get(i).unwrap().clone());

        output.push(execute_equals(lhs_val, rhs_val));
    }

    ExecuteOutput::Array(output)
}

fn execute_equals_array_and_numeric(array: Vec<ExecuteOutput>, numeric: Numeric) -> ExecuteOutput {
    let mut array_numerics: Vec<Numeric> = Vec::new();
    for val in array {
        match val {
            ExecuteOutput::Numeric (x) => array_numerics.push(x),
            other => panic!("Cannot equate non numerics currently. {:?}", other)
        }
    }

    let mut output: Vec<ExecuteOutput> = Vec::new();

    let numeric = ExecuteOutput::Numeric(numeric);
    for val in array_numerics {
        let val = ExecuteOutput::Numeric(val);
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