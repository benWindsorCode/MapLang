use super::structures::ExecuteOutput;
use crate::parse::structures::{Numeric, DyadicVerb};
use super::dyadic_add::execute_add;
use super::dyadic_multiply::execute_multiply;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum IdentityType {
    ADD,
    MULTIPLY
}

pub fn execute_reduce_dyadic_lhs(lhs_verb: DyadicVerb, rhs: ExecuteOutput) -> ExecuteOutput {
    match lhs_verb {
        DyadicVerb::Add => reduce_dyadic_add(rhs),
        DyadicVerb::Multiply => reduce_dyadic_multiply(rhs),
        other => panic!("Cannot reduce over dyadic verb: {:?}", other)
    }
}

fn reduce_dyadic_add(rhs: ExecuteOutput) -> ExecuteOutput {
    match rhs {
        ExecuteOutput::Array (arr) => {
            let first = arr.get(0).unwrap().clone();

            let mut total = initial_reduce_value(first, IdentityType::ADD);

            for val in arr {
                total = execute_add(total, val);
            }

            total
        },
        other => panic!("Cannot reduce add over {:?}", other)
    }
}

fn reduce_dyadic_multiply(rhs: ExecuteOutput) -> ExecuteOutput {
    match rhs {
        ExecuteOutput::Array (arr) => {
            let first = arr.get(0).unwrap().clone();

            let mut total = initial_reduce_value(first, IdentityType::MULTIPLY);

            for val in arr {
                total = execute_multiply(total, val);
            }

            total
        },
        other => panic!("Cannot reduce multiply over {:?}", other)
    }
}

fn initial_reduce_value(template: ExecuteOutput, identity_type: IdentityType) -> ExecuteOutput {
    let identity_val: i64 = match identity_type {
        IdentityType::ADD => 0,
        IdentityType::MULTIPLY => 1
    };

    // Create initial value to start the reduce add operation with
    let initial = match template {
        ExecuteOutput::Numeric (Numeric::Float(_)) => ExecuteOutput::Numeric(Numeric::Float(identity_val as f64)),
        ExecuteOutput::Numeric (Numeric::Int(_)) => ExecuteOutput::Numeric(Numeric::Int(identity_val)),
        ExecuteOutput::Array (arr) => initial_reduce_value_array(arr, identity_type),
        ExecuteOutput::Map (dict) => initial_reduce_value_dict(dict, identity_type),
        other => panic!("Cannot handle dyadic reduce over array of {:?}", other)
    };

    initial
}

fn initial_reduce_value_array(template: Vec<ExecuteOutput>, identity_type: IdentityType) -> ExecuteOutput {
    let mut initial: Vec<ExecuteOutput> = Vec::new();

    for val in template {
        initial.push(initial_reduce_value(val, identity_type));
    }

    ExecuteOutput::Array(initial)
}

fn initial_reduce_value_dict(template: HashMap<String, ExecuteOutput>, identity_type: IdentityType) -> ExecuteOutput {
    let mut initial: HashMap<String, ExecuteOutput> = HashMap::new();

    for (key, val) in template {
        initial.insert(key, initial_reduce_value(val, identity_type));
    }

    ExecuteOutput::Map(initial)
}