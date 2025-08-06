use std::collections::LinkedList;
use std::fmt::{self, Display, Formatter};

use crate::{
    calculation::state::*,
    expression::Expression::{self, Number, Op, Parenthesis},
    operator::{
        Operator::{self, Add, Divide, Multiply, RaisedTo, Subtract},
        ParenType::{Left, Right},
    },
};

#[derive(Debug)]
pub enum NotationType {
    Infix(Vec<Expression<f64>>),
}

impl Display for NotationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut string = String::from("[");
        let x = self.unwrap();
        for i in x {
            string.push(' ');
            match i {
                Number(num) => string.push_str(&format!("{num}")),
                Op(operator) => string.push(match operator {
                    Add => '+',
                    Subtract => '-',
                    Multiply => '×',
                    Divide => '÷',
                    RaisedTo => '^',
                }),
                Parenthesis(paren) => string.push(match paren {
                    Left => '(',
                    Right => ')',
                }),
                // Something => panic!("bruh"),
            }
            string.push(',');
        }
        string.push(']');
        write!(f, "{string}")
    }
}

impl NotationType {
    fn unwrap(&self) -> &Vec<Expression<f64>> {
        match self {
            NotationType::Infix(x) => x,
            // NotationType::Postfix(x) => x,
        }
    }

    pub fn solve(&self) -> CalculationState {
        let mut state = CalculationState::Success(0.0);
        let expression_vec = self.unwrap();
        let mut op_list: LinkedList<Expression<f64>> = LinkedList::new();
        let mut p_list: LinkedList<f64> = LinkedList::new();
        let mut num_count: u8 = 0;
        'solving_expression: for i in expression_vec {
            match i {
                Parenthesis(x) => match x {
                    Left => {
                        op_list.push_front(Parenthesis(Left));
                    }

                    Right => loop {
                        let last_pop = op_list.pop_front().unwrap();
                        match last_pop {
                            Parenthesis(Left) => break,
                            Op(operator) => {
                                let right_operand = p_list.pop_front().unwrap_or_else(|| {
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else(||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Left operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });

                                if let CalculationState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(
                                        left_operand,
                                        operator,
                                        right_operand,
                                    ) {
                                        CalculationState::Success(num) => {
                                            p_list.push_front(num);
                                            state = CalculationState::Success(num);
                                        }
                                        x => {
                                            state = x;
                                            break 'solving_expression;
                                        }
                                    }
                                }
                            }
                            _ => panic!("Bruh what"),
                        }
                    },
                },
                Op(x) => loop {
                    match op_list.front().unwrap() {
                        Op(op) => {
                            if x.has_higher_precedence(op) {
                                op_list.push_front(Op(*x));
                                break;
                            } else {
                                let right_operand = p_list.pop_front().unwrap_or_else(|| {
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else( ||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });

                                if let CalculationState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(left_operand, *op, right_operand)
                                    {
                                        CalculationState::Success(num) => {
                                            p_list.push_front(num);
                                            state = CalculationState::Success(num);
                                        }
                                        x => {
                                            state = x;
                                            break 'solving_expression;
                                        }
                                    };
                                }

                                op_list.pop_front();
                            }
                        }
                        _ => {
                            op_list.push_front(Op(*x));
                            break;
                        }
                    }
                },
                Number(x) => {
                    p_list.push_front(*x);
                    num_count += 1;
                } // Something => todo!("Bro how in the world did'ya even get here"),
            }
        }

        println!("\x1b[33mp_list has : {} elements \x1b[0m", p_list.len());
        match state {
            CalculationState::Success(_num) => {
                let value = p_list.pop_front().unwrap_or(0.0);
                if num_count == 1 {
                    if value == 143.0 {
                        CalculationState::Heart
                    } else if value == 182.0 {
                        CalculationState::Hate
                    } else if value == 404.0 {
                        CalculationState::NotFound
                    } else {
                        CalculationState::Success(value)
                    }
                } else {
                    CalculationState::Success(value)
                }
            }
            x => x,
        }
    }
}

fn solve_binary_expression(left: f64, operator: Operator, right: f64) -> CalculationState {
    match operator {
        Add => CalculationState::Success(left + right),
        Subtract => CalculationState::Success(left - right),
        Multiply => CalculationState::Success(left * right),
        Divide => {
            if right == 0.0 && left == 0.0 {
                CalculationState::Indeterminate
            } else if right == 0.0 {
                CalculationState::Infinite
            } else {
                CalculationState::Success(left / right)
            }
        }
        RaisedTo => {
            if left == 0.0 && right == 0.0 {
                CalculationState::Indeterminate
            } else if right == 0.0 {
                CalculationState::Success(1.0)
            } else {
                CalculationState::Success(left.powf(right))
            }
        }
    }
}
