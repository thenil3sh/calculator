#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    RaisedTo,
}

use Operator::{
    Add,
    Subtract,
    Multiply,
    Divide,
    RaisedTo
};

#[derive(Debug, Copy, Clone)]

pub enum ParenType {
    Left,
    Right,
}

impl Operator {
    pub fn has_higher_precedence(&self, compare_with: &Operator) -> bool {
        let self_score: u8 = match self {
            Add => 0,
            Subtract => 1,
            Multiply => 2,
            Divide => 3,
            RaisedTo => 4,
        };
        let other_score: u8 = match compare_with {
            Add => 0,
            Subtract => 1,
            Multiply => 2,
            Divide => 3,
            RaisedTo => 4,
        };

        self_score > other_score
    }
}

