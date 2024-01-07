use std::{ops::{BitAnd, BitOr}, fmt::Debug};
use nom::{bytes::complete::tag, IResult, combinator::{value, map_res}, branch::alt, sequence::tuple, multi::many0};

use super::Binary;

pub trait Solution {
    fn solution(&self) -> Binary;
}

type Operation = fn(Binary, Binary) -> Binary;

pub struct SubEquation {
    left: Equation,
    operation: Operation,
    right: Equation,
}

impl Debug for SubEquation {
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

impl Default for SubEquation {
    fn default() -> Self {
        Self {
            left: Default::default(),
            operation: Binary::bitand,
            right: Default::default()
        }
    }
}

impl Solution for SubEquation {
    fn solution(&self) -> Binary {
        (self.operation)(self.left.solution(), self.right.solution())
    }
}

#[derive(Debug)]
pub enum Equation {
    Binary(Binary),
    Equation(Box<SubEquation>),
    Inverse(Box<Equation>),
}

impl Default for Equation {
    fn default() -> Self {
        Self::Binary(Binary::High)
    }
}

impl Solution for Equation {
    fn solution(&self) -> Binary {
        match self {
            Equation::Binary(binary) => *binary,
            Equation::Equation(equation) => equation.solution(),
            Equation::Inverse(to_inverse) => !to_inverse.solution(),

        }
    }
}

impl TryFrom<String> for Equation {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = parse_equation(&value);

        match parsed {
            Ok((remaining, equation)) if remaining.is_empty() => Ok(equation),
            _ => Err(())
        }
    }
}


fn parse_equation(input: &str) -> IResult<&str, Equation> {

    let (input, first) = parse_binary(input)?;
    let (input, parts) = many0(equation_right_side)(input)?;

    if parts.is_empty() {
        Ok((input, first))
    } else {
        let mut total = first;

        for (operation, binary) in parts {
            total = Equation::Equation(Box::new(
                SubEquation {
                    left: total,
                    operation,
                    right: binary,
                }
            ))
        }
        
        Ok((input, total))
    }
}


fn parse_binary(input: &str) -> IResult<&str, Equation> {
    let parse_high = value(Binary::High, tag("1"));
    let parse_low = value(Binary::Low, tag("0"));

    map_res(alt((parse_high, parse_low)), |binary| Ok::<Equation, ()>(Equation::Binary(binary)))(input)
}

fn pasre_operation(input: &str) -> IResult<&str, Operation> {
    let parse_and = value(Binary::bitand as Operation, tag("&"));
    let parse_or = value(Binary::bitor as Operation, tag("|"));

    alt((parse_and, parse_or))(input)
}

fn equation_right_side(input: &str) -> IResult<&str, (Operation, Equation)> {
    let mut operation_before_binary = tuple((pasre_operation, parse_binary));
    
    operation_before_binary(input)
}