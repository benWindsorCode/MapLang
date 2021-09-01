use super::structures::ExecuteOutput;
use crate::parse::structures::Numeric;

pub fn execute_greaterthan(lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match (lhs, rhs) {
        (ExecuteOutput::Array (lhs_array), ExecuteOutput::Numeric (rhs_numeric)) => execute_array_greaterthan_numeric(lhs_array, rhs_numeric),
        (ExecuteOutput::Numeric (lhs_numeric), ExecuteOutput::Numeric (rhs_numeric)) => execute_numeric_greaterthan_numeric(lhs_numeric, rhs_numeric),
        (lhs_other, rhs_other) => panic!("Cannot calculate > of {:?} / {:?}", lhs_other, rhs_other)
    }
}

fn execute_array_greaterthan_numeric(lhs_array: Vec<ExecuteOutput>, rhs_numeric: Numeric) -> ExecuteOutput {
    let mut output: Vec<ExecuteOutput> = Vec::new();

    // re-wrap numeric in ExecuteOutput to be passed back into execute_greaterthan
    let rhs_numeric = ExecuteOutput::Numeric(rhs_numeric);
    for val in lhs_array {
        output.push(execute_greaterthan(val, rhs_numeric.clone()));
    }

    ExecuteOutput::Array(output)
}

fn execute_numeric_greaterthan_numeric(lhs_numeric: Numeric, rhs_numeric: Numeric) -> ExecuteOutput {
    // TODO: do we want a bool type?
    let out = if lhs_numeric > rhs_numeric {
        Numeric::Int(1)
    } else {
        Numeric::Int(0)
    };

    ExecuteOutput::Numeric(out)
}