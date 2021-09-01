use std::collections::HashMap;
use std::ops::{Add, Mul};
use std::iter::Sum;

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Node(Box<AstNode>),
    Numeric(Numeric),
    DyadicOp {
        verb: DyadicVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>
    },
    MonadicOp {
        verb: MonadicVerb,
        rhs: Box<AstNode>
    },
    OperatorOp {
        lhs_verb: DyadicVerb,
        operator_verb: OperatorVerb,
        rhs: Box<AstNode>
    },
    Terms(Vec<AstNode>),
    GlobalVar {
        variable: String,
        expression: Box<AstNode>,
    },
    Array(Vec<AstNode>),
    Map(HashMap<String, AstNode>),
    String(String),
    Variable(String)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DyadicVerb {
    Add,
    Subtract,
    Divide,
    Multiply,
    Replicate,
    GreaterThan,
    Access
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MonadicVerb {
    Print,
    Generate,
    Shape
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OperatorVerb {
    Reduce
}

#[derive(Debug, Copy, PartialEq, Clone, PartialOrd)]
pub enum Numeric {
    Int(i64),
    Float(f64)
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a + b),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a + b),
            (self_unknown, other_unknown) => panic!("Cannot add numerics: {:?} + {:?}", self_unknown, other_unknown)
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, other: Numeric) -> Numeric {
        match (self, other) {
            (Numeric::Int(a), Numeric::Int(b)) => Numeric::Int(a * b),
            (Numeric::Float(a), Numeric::Float(b)) => Numeric::Float(a * b),
            (self_unknown, other_unknown) => panic!("Cannot multiply numerics: {:?} + {:?}", self_unknown, other_unknown)
        }
    }
}

impl<'a> Sum<Numeric> for Numeric {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Numeric>, 
    {
        let mut int_vals: Vec<i64> = Vec::new();
        let mut float_vals: Vec<f64> = Vec::new();

        for val in iter {
            match val {
                Numeric::Int (x) => int_vals.push(x),
                Numeric::Float (x) => float_vals.push(x)
            }
        }

        if int_vals.len() > 0 && float_vals.len() > 0 {
            panic!("Cannot fold Numeric with float and ints");
        }

        if int_vals.len() > 0 {
            return Numeric::Int(int_vals.into_iter().sum());
        }

        if float_vals.len() > 0 {
            return Numeric::Float(float_vals.into_iter().sum())
        }

        panic!("Couldn't fold Numeric");
    }
}