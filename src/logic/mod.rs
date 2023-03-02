use core::fmt;
use std::ops::{BitAnd, BitOr, Not, Add};

pub mod equation;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Binary(Binary),
    #[default]
    None,
    Undefined,
}

impl Not for Signal {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Binary(binary) => Self::Binary(!binary),
            _ => Self::Undefined,
        }
    }
}

impl BitAnd for Signal {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Binary(bin_self), Self::Binary(bin_rhs)) => Self::Binary(bin_self & bin_rhs),
            _ => Self::Undefined,
        }
    }
}

impl BitOr for Signal {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Binary(bin_self), Self::Binary(bin_rhs)) => Self::Binary(bin_self | bin_rhs),
            _ => Self::Undefined,
        }
    }
}

impl Add for Signal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Binary(_), Self::Binary(_)) => Self::Undefined,
            (Self::None, Self::Binary(binary)) => Self::Binary(binary),
            (Self::Binary(binary), Self::None) => Self::Binary(binary),
            _ => Self::Undefined,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
    #[default]
    Low,
    High,
}

impl Not for Binary {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

impl BitAnd for Binary {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::High, Self::High) => Self::High,
            _ => Self::Low,
        }
    }
}

impl BitOr for Binary {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Low, Self::Low) => Self::Low,
            _ => Self::High,
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Binary::High => write!(f, "High"),
            Binary::Low => write!(f, "Low"),
        }
    }
}