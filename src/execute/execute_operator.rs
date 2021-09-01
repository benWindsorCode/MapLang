use super::structures::ExecuteOutput;
use crate::parse::structures::{DyadicVerb, OperatorVerb};
use super::operator_reduce::execute_reduce_dyadic_lhs;

pub fn execute_operator_op(lhs_verb: DyadicVerb, operator_verb: OperatorVerb, rhs: ExecuteOutput) -> ExecuteOutput {
    match operator_verb {
        OperatorVerb::Reduce => {
            execute_reduce_dyadic_lhs(lhs_verb, rhs)
        },
        other => panic!("Operator verb not implemented {:?}", other)
    }
}