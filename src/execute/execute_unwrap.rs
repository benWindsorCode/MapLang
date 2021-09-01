use crate::parse::structures::AstNode;
use super::structures::ExecuteOutput;
use super::execute::execute_expression;
use std::collections::HashMap;


pub fn unwrap_array(vals: Vec<AstNode>, state: &mut HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    // TODO: how do we want to handle mixed arrays?
    let mut array: Vec<ExecuteOutput> = Vec::new();

    for val in vals {
        match val {
            AstNode::Numeric (x) => {
                array.push(ExecuteOutput::Numeric(x))
            },
            AstNode::Array (arr) => {
                array.push(unwrap_array(arr, state))
            },
            AstNode::Dictionary (dict_val) => {
                array.push(unwrap_dictionary(dict_val, state))
            }
            other => panic!("cant handle array of: {:?}", other)
        };
    };

    return ExecuteOutput::Array(array)
}

pub fn unwrap_dictionary(dict: HashMap<String, AstNode>, state: &mut HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let mut unwrapped_dict: HashMap<String, ExecuteOutput> = HashMap::new();

    for (key, value) in dict {
        unwrapped_dict.insert(key, execute_expression(value, state));
    }

    ExecuteOutput::Dictionary(unwrapped_dict)
}

// Given a variable name, unwrap its value, copy the data from state and return a new execute output
pub fn unwrap_variable(var: String, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let var_value = match state.get(&var).unwrap() {
        ExecuteOutput::Array (arr) => {
            ExecuteOutput::Array(arr.clone())
        },
        ExecuteOutput::Dictionary (dict) => {
            let mut copied_dict: HashMap<String, ExecuteOutput> = HashMap::new();

            for (key, values) in dict {
                copied_dict.insert(key.to_string(), values.clone());
            }

            ExecuteOutput::Dictionary(copied_dict)
        },
        ExecuteOutput::Numeric (int_val) => {
            ExecuteOutput::Numeric(int_val.clone())
        },
        other => panic!("Cant handle variables of type {:?}", other)
    };

    var_value
}