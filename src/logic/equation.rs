use std::{ops::{BitAnd, BitOr}, fmt::Debug};
use nom::{bytes::complete::tag, IResult, combinator::value, branch::alt, sequence::tuple};

use super::Binary;

pub trait Solution {
    fn solution(&self) -> Binary;
}

type Operation = fn(Binary, Binary) -> Binary;

pub struct Equation {
    left: EquationPart,
    operation: Operation,
    right: EquationPart,
}

impl Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let value = if self.operation == Binary::bitand {
            "bitand".to_string()
        } else if self.operation == Binary::bitor {
            "bitor".to_string()
        } else {
            "shit".to_string()
        };


        f.debug_struct("Equation")
            .field("left", &self.left)
            .field("operation", &value)
            .field("right", &self.right)
            .finish()
    }
}

impl TryFrom<String> for Equation {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = parse_equation(&value);
        
        match parsed {
            Ok(result) if result.0.is_empty() => {
                dbg!(&result.1);
                Ok(result.1)
            },
            _ => Err(())
        }
    }
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, left) = parse_binary(&input)?;
    let (input, operation) = pasre_operation(&input)?;
    let (input, right) = parse_equation_part(&input)?;
    
    Ok((input, Equation {
        left: EquationPart::Binary(left),
        operation,
        right
    }))
}

fn parse_equation_part(input: &str) -> IResult<&str, EquationPart> {
    let (input, first) = parse_binary(input)?;

    if input.is_empty() {
        Ok((input, EquationPart::Binary(first)))
    } else {

        let result = tuple((pasre_operation, parse_equation_part))(input);

        match result {
            Ok((input, (operation, right))) => {
                Ok((input, EquationPart::Equation(
                    Box::new(Equation {
                        left: EquationPart::Binary(first),
                        operation,
                        right
                    })
                )))
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}

fn parse_binary(input: &str) -> IResult<&str, Binary> {
    let parse_high = value(Binary::High, tag("1"));
    let parse_low = value(Binary::Low, tag("0"));

    alt((parse_high, parse_low))(input)
}

fn pasre_operation(input: &str) -> IResult<&str, Operation> {
    let parse_and = value(Binary::bitand as Operation, tag("&"));
    let parse_or = value(Binary::bitor as Operation, tag("|"));

    alt((parse_and, parse_or))(input)
}

impl Default for Equation {
    fn default() -> Self {
        Self {
            left: Default::default(),
            operation: Binary::bitand,
            right: Default::default()
        }
    }
}

impl Solution for Equation {
    fn solution(&self) -> Binary {
        (self.operation)(self.left.solution(), self.right.solution())
    }
}

#[derive(Debug)]
pub enum EquationPart {
    Binary(Binary),
    Equation(Box<Equation>),
    Inverse(Box<EquationPart>),
}

impl Default for EquationPart {
    fn default() -> Self {
        Self::Binary(Binary::High)
    }
}

impl Solution for EquationPart {
    fn solution(&self) -> Binary {
        match self {
            EquationPart::Binary(binary) => *binary,
            EquationPart::Equation(equation) => equation.solution(),
            EquationPart::Inverse(to_inverse) => !to_inverse.solution(),

        }
    }
}