use std::collections::HashMap;
use crate::parse::structures::AstNode;
use super::structures::ExecuteOutput;
use super::execute_dyadic::execute_dyadic_op;
use super::execute_monadic::execute_monadic_op;
use super::execute_operator::execute_operator_op;
use super::execute_unwrap::*;

pub fn execute_expression(expression: AstNode, state: &mut HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    match expression {
        // Unwrap lhs and rhs and compute operation
        AstNode::DyadicOp {verb, lhs, rhs} => {
            let lhs = execute_expression(*lhs, state);
            let rhs = execute_expression(*rhs, state);
            execute_dyadic_op(verb, lhs, rhs)
        },
        // Unwrap rhs and compute operation
        AstNode::MonadicOp {verb, rhs} => {
            let rhs = execute_expression(*rhs, state);
            execute_monadic_op(verb, rhs)
        },
        AstNode::OperatorOp {lhs_verb, operator_verb, rhs} => {
            let rhs = execute_expression(*rhs, state);
            execute_operator_op(lhs_verb, operator_verb, rhs)
        },
        // Unwrap + compute the inner values of the array
        AstNode::Array (vals) => {
            unwrap_array(vals, state)
        },
        // Fetch var from state and copy + return
        AstNode::Variable (var) => {
            unwrap_variable(var, state)
        },
        AstNode::Numeric (val) => {
            ExecuteOutput::Numeric(val)
        },
        AstNode::String (val) => {
            ExecuteOutput::String(val)
        },
        AstNode::Map (dict) => {
            unwrap_dictionary(dict, state)
        },
        AstNode::GlobalVar {variable, expression} => {
            // TODO: is this nice? it stops a mutable borrow of the state twice
            //        however what if the copy_state is updated inside execute_expression
            //        we shoudl deal with that here somehow?
            let mut copy_state = state.clone();
            state.insert(variable, execute_expression(*expression, &mut copy_state));

            // TODO Merge the inner and outer states? only needed once inner executions can modify state
            ExecuteOutput::Null
        },
        other_matched => panic!("Couldn't match node {:?} in execute expression", other_matched)
    }
}