use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::Error;

/// A JSON number.
/// Can be a `u64`, `i64` or `f64`.
#[derive(Debug, Clone, PartialOrd)]
pub enum Number {
    /// An unsigned integer.
    /// (u64)
    UInt(u64),
    /// A signed integer.
    /// (i64)
    Int(i64),
    /// A floating point number.
    /// (f64)
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UInt(x) => write!(f, "{x}"),
            Self::Int(x) => write!(f, "{x}"),
            Self::Float(x) => write!(f, "{x}"),
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Number::UInt(x) => x.hash(state),
            Number::Int(x) => x.hash(state),
            Number::Float(x) => x.to_bits().hash(state),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UInt(l0), Self::UInt(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            return Ok(Number::Float(s.parse::<f64>()?));
        }

        if s.starts_with('-') {
            return Ok(Number::Int(s.parse::<i64>()?));
        }

        Ok(Number::UInt(s.parse::<u64>()?))
    }
}

impl Eq for Number {}
