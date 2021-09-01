use std::collections::HashMap;
use crate::parse::structures::Numeric;

#[derive(Debug, Clone)]
pub enum ExecuteOutput {
    // Array of any value
    Array(Vec<ExecuteOutput>),
    // Map of string -> any value
    Map(HashMap<String, ExecuteOutput>), 
    // Numeric int or float
    Numeric(Numeric),
    // General string
    String(String),
    Null
}