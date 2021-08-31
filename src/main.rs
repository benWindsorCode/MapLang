extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use std::fs;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "array_language_grammar.pest"]
struct ArrayLanguageParser;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DyadicVerb {
    Add,
    Replicate,
    GreaterThan
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MonadicVerb {
    Print,
    Generate,
    Shape
}

#[derive(Debug, Clone)]
pub enum ExecuteOutput {
    // Array of ints
    IntArray(Vec<i32>),
    // TODO: is there a nicer way to make the inner value forced to Dict? perhaps the Dict(Dict) pattern with inner structure of enum
    // Array of dicts
    DictArray(Vec<ExecuteOutput>),
    // Dict of int arrays
    Dictionary(HashMap<String, Vec<i32>>),
    // Dict of ints
    IntDictionary(HashMap<String, i32>),
    // Integer
    Integer(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Node(Box<AstNode>),
    Integer(i32),
    DoublePrecisionFloat(f64),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>
    },
    MonadicOp {
        verb: MonadicVerb,
        rhs: Box<AstNode>
    },
    Terms(Vec<AstNode>),
    GlobalVar {
        variable: String,
        expression: Box<AstNode>,
    },
    Array(Vec<AstNode>),
    Dictionary(HashMap<String, AstNode>),
    Variable(String)
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = ArrayLanguageParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => {
                ast.push(AstNode::Node(Box::new(build_ast_from_expr(pair))));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::expression => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::dyadicExpression => {
            let mut pair = pair.into_inner();

            let lhs = pair.next().unwrap();
            let lhs = build_ast_from_expr(lhs);

            let verb = pair.next().unwrap();
                    
            let rhs = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhs);

            parse_dyadic_verb(lhs, verb, rhs)
        },
        Rule::monadicExpression => {
            let mut pair = pair.into_inner();

            let verb = pair.next().unwrap();
            let rhs = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhs);

            parse_monadic_verb(verb, rhs)
        },
        Rule::assignment => {
            let mut pair = pair.into_inner();
            let variable = pair.next().unwrap();
            let expression = pair.next().unwrap();
            let expression = build_ast_from_expr(expression);
            AstNode::GlobalVar {
                variable: String::from(variable.as_str()),
                expression: Box::new(expression)
            }
        },
        Rule::terms => {
            let terms: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();

            // If single item, then unwrap it from vector
            match terms.len() {
                1 => terms.get(0).unwrap().clone(),
                _ => AstNode::Terms(terms)
            }
        },
        unknown_expr => panic!("Not implemented: {:?}", unknown_expr)
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {
            let istr = pair.as_str();
            let integer: i32 = istr.parse().unwrap();
            AstNode::Integer(integer)
        },
        Rule::array => {
            let vals: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();

            AstNode::Array(vals)
        },
        Rule::dictionary => {
            let mut dictionary: HashMap<String, AstNode> = HashMap::new();
            
            for entry in pair.into_inner() {
                let mut entry = entry.into_inner();

                // Get variable name, strip first and last char as they are "'" characters
                let mut var = entry.next().unwrap().as_str().chars();
                var.next();
                var.next_back();
                let var = var.as_str().to_string();

                let expr = entry.next().unwrap();
                let expr = build_ast_from_expr(expr);
                
                dictionary.insert(var, expr);
            }

            AstNode::Dictionary(dictionary)
        },
        Rule::expression => {
            build_ast_from_expr(pair)
        },
        Rule::variable => {
            AstNode::Variable(pair.as_str().to_string())
        },
        unknown_term => panic!("Unexpected term: {:?}", unknown_term)
    }
}

fn parse_dyadic_verb(lhs: AstNode, pair: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::DyadicOp {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
        verb: match pair.as_str() {
            "+" => DyadicVerb::Add,
            "/" => DyadicVerb::Replicate,
            ">" => DyadicVerb::GreaterThan,
            _ => panic!("Verb not implemented")
        }
    }
}

fn parse_monadic_verb(pair: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::MonadicOp {
        rhs: Box::new(rhs),
        verb: match pair.as_str() {
            "print" => MonadicVerb::Print,
            "⍳" => MonadicVerb::Generate,
            "⍴" => MonadicVerb::Shape,
            other => panic!("Verb '{}' not implemented", other)
        }
    }
}

fn run_program(program: Vec<AstNode>) {
    let mut state: HashMap<String, ExecuteOutput> = HashMap::new();

    for line in program {
        let node = match line {
            AstNode::Node(inner) => {
                *inner
            },
            node_matched => panic!("Unexpected node type in run program {:?}", node_matched)
        };

        match node {
            AstNode::DyadicOp {verb, lhs, rhs} => {
                println!("Executing dyadic Op {:?}", verb);
                let lhs_node = *lhs;
                let rhs_node = *rhs;
                execute_diadic_op(verb, lhs_node, rhs_node, &state);
            },
            AstNode::MonadicOp {verb, rhs} => {
                println!("Executing monadic Op {:?}", verb);
                let rhs_node = *rhs;
                execute_monadic_op(verb, rhs_node, &state);
            },
            AstNode::GlobalVar {variable, expression} => {
                state.insert(variable, execute_expression(*expression, &state));
            },
            node_matched => panic!("Couldn't match node {:?}", node_matched)
        };

        println!("State is: {:?}", state);
    }
}

fn execute_diadic_op(verb: DyadicVerb, lhs: AstNode, rhs: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    match verb {
        DyadicVerb::Add => {
            execute_add_arrays(lhs, rhs, state)
        },
        DyadicVerb::Replicate => {
            execute_replicate_array(lhs, rhs, state)
        },
        DyadicVerb::GreaterThan => {
            execute_array_greaterthan_int(lhs, rhs, state)
        }
        other => panic!("Dyadic verb not implemented {:?}", other)
    }
}

fn execute_monadic_op(verb: MonadicVerb, rhs: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    match verb {
        MonadicVerb::Print => {
            let val_to_print = execute_expression(rhs, state);
            println!("PRINT {:?}", val_to_print);
            ExecuteOutput::IntArray(Vec::new())
        },
        MonadicVerb::Generate => {
            let expression = execute_expression(rhs, state);
            let size = match expression {
                ExecuteOutput::IntArray (arr) => arr.get(0).unwrap().clone(),
                ExecuteOutput::Integer (_) => 0,
                other => panic!("Cant handle {:?} in monadic op", other)
            };

            let mut generated: Vec<i32> = Vec::new();

            for i in 0..size {
                generated.push(i);
            }

            ExecuteOutput::IntArray(generated)
        },
        MonadicVerb::Shape => {
            let expression_size = match execute_expression(rhs, state) {
                ExecuteOutput::IntArray (arr) => arr.len() as i32,
                other => panic!("Cant handle {:?} in monadic shape", other)
            };

            let mut output: Vec<i32> = Vec::new();
            output.push(expression_size);

            // TODO: replace this with an outupt of int or array depending on shape of object
            ExecuteOutput::IntArray(output)
        }
    }
}

fn execute_expression(expression: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    match expression {
        AstNode::DyadicOp {verb, lhs, rhs} => {
            let lhs_node = *lhs;
            let rhs_node = *rhs;
            execute_diadic_op(verb, lhs_node, rhs_node, state)
        },
        AstNode::MonadicOp {verb, rhs} => {
            let rhs_node = *rhs;
            execute_monadic_op(verb, rhs_node, state)
        },
        AstNode::Array (vals) => {
            unwrap_array(vals, state)
        },
        AstNode::Variable (var) => {
            let var_value = match state.get(&var).unwrap() {
                ExecuteOutput::IntArray (arr) => arr,
                other => panic!("Cant handle variables of non array type {:?}", other)
            };

            let mut copied_var: Vec<i32> = Vec::new();
            for val in var_value {
                copied_var.push(*val);
            }

            ExecuteOutput::IntArray(copied_var)
        },
        AstNode::Integer (val) => {
            ExecuteOutput::Integer(val)
        },
        AstNode::Dictionary (dict) => {
            unwrap_dictionary(dict, state)
        },
        other_matched => panic!("Couldn't match node {:?} in execute expression", other_matched)
    }
}

fn execute_add_arrays(lhs: AstNode, rhs: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let lhs_array = match execute_expression(lhs, state) {
        ExecuteOutput::IntArray (arr) => arr,
        other => panic!("Array addition cant handle non array type {:?}", other)
    };

    let rhs_array = match execute_expression(rhs, state) {
        ExecuteOutput::IntArray (arr) => arr,
        other => panic!("Array addition cant handle non array type {:?}", other)
    };

    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot add arrays of two different sizes: {} vs {}", lhs_array.len(), rhs_array.len());
    }

    println!("Adding arrays: {:?} + {:?}", lhs_array, rhs_array);

    let mut output: Vec<i32> = Vec::new();

    for i in 0..lhs_array.len() {
        output.push(lhs_array.get(i).unwrap() + rhs_array.get(i).unwrap());
    }

    println!("Result of addition: {:?}", output);
    ExecuteOutput::IntArray(output)
}

fn execute_replicate_array(lhs: AstNode, rhs: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let lhs_array = match execute_expression(lhs, state) {
        ExecuteOutput::IntArray (arr) => arr,
        other => panic!("Array replication cant handle non array type {:?}", other)
    };

    let rhs_array = match execute_expression(rhs, state) {
        ExecuteOutput::IntArray (arr) => arr,
        other => panic!("Array replication cant handle non array type {:?}", other)
    };

    if lhs_array.len() != rhs_array.len() {
        panic!("Cannot add arrays of two different sizes: {} vs {}", lhs_array.len(), rhs_array.len());
    }

    println!("Replicating: {:?} / {:?}", lhs_array, rhs_array);

    let mut output: Vec<i32> = Vec::new();

    for i in 0..lhs_array.len() {
        let multiplicity = lhs_array.get(i).unwrap();

        if *multiplicity < 0 {
            panic!("Multiplicity {} is less than zero, not allowed in replicate command", multiplicity);
        }

        for j in 0..*multiplicity {
            output.push(rhs_array.get(i).unwrap().clone());
        }
    }

    ExecuteOutput::IntArray(output)
}

fn execute_array_greaterthan_int(lhs: AstNode, rhs: AstNode, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let lhs_array = match execute_expression(lhs, state) {
        ExecuteOutput::IntArray (arr) => arr,
        other => panic!("Array greaterthan cant handle non array lhs type {:?}", other)
    };

    let rhs_integer = match execute_expression(rhs, state) {
        ExecuteOutput::Integer (val) => val,
        other => panic!("Array greaterthan cant handle non integer rhs type {:?}", other)
    };

    let mut output: Vec<i32> = Vec::new();

    for val in lhs_array {
        if val > rhs_integer {
            output.push(1)
        } else {
            output.push(0)
        }
    }

    ExecuteOutput::IntArray(output)
}

// For now only support vectors of integers
fn unwrap_array(vals: Vec<AstNode>, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let mut int_array: Vec<i32> = Vec::new();
    let mut dict_array: Vec<ExecuteOutput> = Vec::new();

    for val in vals {
        let val_to_push = match val {
            AstNode::Integer (int_val)  => {
                int_array.push(int_val)
            },
            AstNode::Dictionary (dict_val) => {
                dict_array.push(unwrap_dictionary(dict_val, state))
            }
            other => panic!("cant handle array of: {:?}", other)
        };
    };

    if int_array.len() > 0 && dict_array.len() > 0 {
        panic!("Cannot have array of ints and dicts");
    }

    if int_array.len() > 0 {
        return ExecuteOutput::IntArray(int_array);
    }

    // TODO: what if both arrays are size zero? then we return dict array by default?
    return ExecuteOutput::DictArray(dict_array)
}

fn unwrap_dictionary(dict: HashMap<String, AstNode>, state: &HashMap<String, ExecuteOutput>) -> ExecuteOutput {
    let mut dict_of_vec: HashMap<String, Vec<i32>> = HashMap::new();
    let mut dict_of_int: HashMap<String, i32> = HashMap::new();

    for (key, value) in dict {
        match execute_expression(value, state) {
            ExecuteOutput::IntArray (arr) => { dict_of_vec.insert(key, arr); },
            ExecuteOutput::Integer (int_val) => { dict_of_int.insert(key, int_val); },
            other => panic!("Cant support dictionary with value {:?}", other)
        };
    }

    if dict_of_vec.len() > 0 && dict_of_int.len() > 0 {
        panic!("Cannot create dict of vec and int");
    }

    if dict_of_vec.len() > 0 {
        return ExecuteOutput::Dictionary(dict_of_vec);
    }

    // TODO: if both dict of vec and int are empty then we default to int dict is that nice?
    ExecuteOutput::IntDictionary(dict_of_int)
}

fn main() {
    let unparsed_file = fs::read_to_string("test_program_2.txt").expect("cannot read file");

    println!("Raw file:\n{:?}", unparsed_file);

    let file = ArrayLanguageParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse")
        .next().unwrap();

    println!("{:?}", file);

    let out = parse(&unparsed_file).unwrap();

    println!("{:?}", out);

    run_program(out);
}

