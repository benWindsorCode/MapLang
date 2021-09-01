use crate::parse::structures::DyadicVerb;
use super::structures::ExecuteOutput;
use super::dyadic_add::execute_add;
use super::dyadic_divide::execute_divide;
use super::dyadic_greaterthan::execute_greaterthan;
use super::dyadic_replicate::execute_replicate;


pub fn execute_dyadic_op(verb: DyadicVerb, lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match verb {
        DyadicVerb::Add => {
            execute_add(lhs, rhs)
        },
        DyadicVerb::Divide => {
            execute_divide(lhs, rhs)
        },
        DyadicVerb::Replicate => {
            execute_replicate(lhs, rhs)
        },
        DyadicVerb::GreaterThan => {
            execute_greaterthan(lhs, rhs)
        },
        other => panic!("Dyadic verb not implemented {:?}", other)
    }
}