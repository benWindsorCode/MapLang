use crate::parse::structures::DyadicVerb;
use super::structures::ExecuteOutput;
use super::dyadic_add::execute_add;
use super::dyadic_divide::execute_divide;
use super::dyadic_greaterthan::execute_greaterthan;
use super::dyadic_replicate::execute_replicate;
use super::dyadic_multiply::execute_multiply;
use super::dyadic_access::execute_access;

pub fn execute_dyadic_op(verb: DyadicVerb, lhs: ExecuteOutput, rhs: ExecuteOutput) -> ExecuteOutput {
    match verb {
        DyadicVerb::Add => {
            execute_add(lhs, rhs)
        },
        DyadicVerb::Divide => {
            execute_divide(lhs, rhs)
        },
        DyadicVerb::Multiply => {
            execute_multiply(lhs, rhs)
        }
        DyadicVerb::Replicate => {
            execute_replicate(lhs, rhs)
        },
        DyadicVerb::GreaterThan => {
            execute_greaterthan(lhs, rhs)
        },
        DyadicVerb::Access => {
            execute_access(lhs, rhs)
        },
        other => panic!("Dyadic verb not implemented {:?}", other)
    }
}