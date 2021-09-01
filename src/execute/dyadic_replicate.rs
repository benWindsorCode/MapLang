use super::structures::ExecuteOutput;
use crate::parse::structures::Numeric;

pub fn execute_replicate(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Array (rhs_array)) => execute_replicate_arrays(lhs_array, rhs_array),
        (lhs_other, rhs_other) => panic!("Cannot replicate {:?} / {:?}", lhs_other, rhs_other)
    }
}

fn execute_replicate_arrays(lhs_array: Vec<ExecuteOutput>, rhs_array: Vec<ExecuteOutput>) -> ExecuteOutput {
    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot replcate arrays of different lengths {:?} vs {:?}", lhs_array.len(), rhs_array.len());
    }

    let lhs_array: Vec<i64> = lhs_array.into_iter().map(|x| match x {
        ExecuteOutput::Numeric(Numeric::Int(x_int)) => x_int,
        other => panic!("Cannot replicate with {:?} values on lhs, must be array of ints as lhs", other)
    }).collect();

    let mut output = Vec::new();

    for i in 0..lhs_array.len() {
        let multiplicity = lhs_array.get(i).unwrap();

        if *multiplicity < 0 {
            panic!("Multiplicity {} is less than zero, not allowed in replicate command", multiplicity);
        }

        for _ in 0..*multiplicity {
            output.push(rhs_array.get(i).unwrap().clone());
        }
    }

    ExecuteOutput::Array(output)
}