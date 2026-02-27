use std::fmt::{self, Display, Formatter};

use crate::calculation::result::*;

pub enum CalculationState {
    Success(f64),
    //Imaginary,
    Infinite,
    Indeterminate,
    Heart,
    Hate,
    Invalid,
    NotFound,
}

impl Display for CalculationState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Success(num) => num.approximate().to_string(),
                Self::Infinite => String::from("Infinite"),
                Self::Indeterminate => String::from("Indeterminate"),
                Self::Hate => String::from(":( "),
                Self::Heart => String::from("❤ "),
                Self::Invalid => String::from("Format Error"),
                Self::NotFound => String::from("Not found"),
            }
        )
    }
}