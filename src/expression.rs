use crate::operator::{Operator, ParenType};

#[derive(Debug, Copy, Clone)]
pub enum Expression<T: Into<f64>> {
    Number(T),
    Parenthesis(ParenType),
    Op(Operator),
    // Something,
}
