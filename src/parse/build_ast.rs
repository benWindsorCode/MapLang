use std::collections::HashMap;
use super::structures::*;

#[derive(Parser)]
#[grammar = "language_grammar.pest"]
pub struct ArrayLanguageParser;

pub fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> AstNode {
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
        Rule::operatorExpression => {
            let mut pair = pair.into_inner();

            let lhs_verb = pair.next().unwrap();
            
            let operator_verb = pair.next().unwrap();

            let rhs = pair.next().unwrap();
            let rhs = build_ast_from_expr(rhs);

            parse_operator_verb(lhs_verb, operator_verb, rhs)
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
        unknown_expr => panic!("Not implemented: {:?}, details: {:?}", unknown_expr, pair)
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {
            let istr = pair.as_str();
            let integer: i64 = istr.parse().unwrap();
            AstNode::Numeric(Numeric::Int(integer))
        },
        Rule::decimal => {
            let fstr = pair.as_str();
            let float: f64 = fstr.parse().unwrap();
            AstNode::Numeric(Numeric::Float(float))
        },
        Rule::string => {
            // String first and last char as they are "'" characters
            let mut chars = pair.as_str().chars();
            chars.next();
            chars.next_back();

            AstNode::String(chars.as_str().to_string())
        },
        Rule::array => {
            let vals: Vec<AstNode> = pair.into_inner().map(build_ast_from_term).collect();

            AstNode::Array(vals)
        },
        Rule::map => {
            let mut map: HashMap<String, AstNode> = HashMap::new();
            
            for entry in pair.into_inner() {
                let mut entry = entry.into_inner();

                // Get variable name, strip first and last char as they are "'" characters
                let mut var = entry.next().unwrap().as_str().chars();
                var.next();
                var.next_back();
                let var = var.as_str().to_string();

                let expr = entry.next().unwrap();
                let expr = build_ast_from_expr(expr);
                
                map.insert(var, expr);
            }

            AstNode::Map(map)
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
        verb: dyadic_verb_from_str(pair.as_str())
    }
}

fn dyadic_verb_from_str(verb_str: &str) -> DyadicVerb {
    match verb_str {
        "+" => DyadicVerb::Add,
        "-" => DyadicVerb::Subtract,
        "/" => DyadicVerb::Replicate,
        ">" => DyadicVerb::GreaterThan,
        "÷" => DyadicVerb::Divide,
        "×" => DyadicVerb::Multiply,
        "." => DyadicVerb::Access,
        "=" => DyadicVerb::Equals,
        other => panic!("Dyadic Verb {:?} not implemented", other)
    }
}

fn parse_monadic_verb(pair: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::MonadicOp {
        rhs: Box::new(rhs),
        verb: match pair.as_str() {
            "print" => MonadicVerb::Print,
            "⍳" => MonadicVerb::Generate,
            "⍴" => MonadicVerb::Shape,
            other => panic!("Monadic Verb '{}' not implemented", other)
        }
    }
}

fn parse_operator_verb(lhs_verb_pair: pest::iterators::Pair<Rule>, operator_verb_pair: pest::iterators::Pair<Rule>, rhs: AstNode) -> AstNode {
    AstNode::OperatorOp {
        lhs_verb: dyadic_verb_from_str(lhs_verb_pair.as_str()),
        operator_verb: match operator_verb_pair.as_str() {
            "/" => OperatorVerb::Reduce,
            other => panic!("Operator Verb '{}' not implemented", other)
        },
        rhs: Box::new(rhs)
    }
}