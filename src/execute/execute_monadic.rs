use super::structures::ExecuteOutput;
use crate::parse::structures::MonadicVerb;
use crate::parse::structures::Numeric;

pub fn execute_monadic_op(verb: MonadicVerb, rhs: ExecuteOutput) -> ExecuteOutput {
    match verb {
        MonadicVerb::Print => {
            println!("PRINT {:?}", rhs);

            ExecuteOutput::Null
        },
        MonadicVerb::Generate => {
            let size = match rhs {
                ExecuteOutput::Numeric (Numeric::Int(int_val)) => int_val,
                other => panic!("Cant handle {:?} in monadic generate op", other)
            };

            let mut generated: Vec<ExecuteOutput> = Vec::new();

            for i in 0..size {
                generated.push(ExecuteOutput::Numeric(Numeric::Int(i as i64)));
            }

            ExecuteOutput::Array(generated)
        },
        MonadicVerb::Shape => {
            // TODO: what should 'shape' of array of dicts and dicts be?
            let expression_size = match rhs {
                ExecuteOutput::Array (arr) => arr.len() as i64,
                other => panic!("Cant handle {:?} in monadic shape", other)
            };

            // TODO: replace this with an outupt of int or array depending on shape of object
            ExecuteOutput::Numeric(Numeric::Int(expression_size))
        }
    }
}