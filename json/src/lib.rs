//! A JSON parser and serializer.

mod error;
mod number;
mod parser;
mod value;
pub use error::Error;
pub use number::Number;
pub use value::Value;

#[cfg(test)]
mod test;
