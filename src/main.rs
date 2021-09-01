mod parse;
mod execute;

use parse::structures::AstNode;
use parse::build_ast::{Rule, ArrayLanguageParser, build_ast_from_expr};
use execute::structures::ExecuteOutput;
use execute::execute::execute_expression;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use std::fs;
use std::collections::HashMap;

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    // Recursively build up tree to be executed
    let pairs = ArrayLanguageParser::parse(Rule::program, source)?;

    println!("{:?}", pairs);

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

fn run_program(program: Vec<AstNode>) {
    let mut state: HashMap<String, ExecuteOutput> = HashMap::new();

    // run over nodes 'line by line' executing each line
    for line in program {
        let node = match line {
            AstNode::Node(inner) => {
                *inner
            },
            node_matched => panic!("Unexpected node type in run program {:?}", node_matched)
        };

        // TODO: should we handle the assignment of variables out here instead? separately, then pass an immutable state in...
        execute_expression(node, &mut state);
    }
}

fn main() {
    let unparsed_file = fs::read_to_string("test_program_2.txt").expect("cannot read file");

    let out = parse(&unparsed_file).unwrap();

    println!("{:?}", out);

    run_program(out);
}

