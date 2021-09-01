use super::structures::ExecuteOutput;
use crate::parse::structures::{Numeric, DyadicVerb};
use super::dyadic_add::execute_add;
use std::collections::HashMap;

pub fn execute_reduce_dyadic_lhs(lhs_verb: DyadicVerb, rhs: ExecuteOutput) -> ExecuteOutput {
    match lhs_verb {
        DyadicVerb::Add => reduce_dyadic_add(rhs),
        other => panic!("Cannot reduce over dyadic verb: {:?}", other)
    }
}

fn reduce_dyadic_add(rhs: ExecuteOutput) -> ExecuteOutput {
    match rhs {
        ExecuteOutput::Array (arr) => {
            let first = arr.get(0).unwrap().clone();

            let mut total = initial_reduce_add_value(first);

            for val in arr {
                total = execute_add(total, val);
            }

            total
        },
        other => panic!("Cannot reduce add over {:?}", other)
    }
}

fn initial_reduce_add_value(template: ExecuteOutput) -> ExecuteOutput {
    // Create initial value to start the reduce add operation with
    let initial = match template {
        ExecuteOutput::Numeric (Numeric::Float(_)) => ExecuteOutput::Numeric(Numeric::Float(0.0)),
        ExecuteOutput::Numeric (Numeric::Int(_)) => ExecuteOutput::Numeric(Numeric::Int(0)),
        ExecuteOutput::Array (arr) => initial_reduce_add_value_array(arr),
        ExecuteOutput::Dictionary (dict) => initial_reduce_add_value_dict(dict),
        other => panic!("Cannot handle dyadic reduce over array of {:?}", other)
    };

    initial
}

fn initial_reduce_add_value_array(template: Vec<ExecuteOutput>) -> ExecuteOutput {
    let mut initial: Vec<ExecuteOutput> = Vec::new();

    for val in template {
        initial.push(initial_reduce_add_value(val));
    }

    ExecuteOutput::Array(initial)
}

fn initial_reduce_add_value_dict(template: HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let mut initial: HashMap<String, ExecuteOutput> = HashMap::new();

    for (key, val) in template {
        initial.insert(key, initial_reduce_add_value(val));
    }

    ExecuteOutput::Dictionary(initial)
}