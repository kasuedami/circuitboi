use std::ops::BitAnd;

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

impl TryFrom<String> for Equation {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Err(())
    }
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
        Binary::High
    }
}