use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    str::FromStr,
};

use crate::{parser::Parser, Error, Number};

/// A JSON element.
/// Can be a null, bool, number, string, array or object.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Value {
    /// A null value.
    Null,
    /// A boolean value.
    /// `true` or `false`.
    Bool(bool),
    /// A number value.
    /// Uses `f64` internally.
    Number(Number),
    /// A string value.
    String(String),
    /// An array value.
    /// Contains a vector of `Value`s.
    Array(Vec<Value>),
    /// An object value.
    /// Contains a map of `String`s to `Value`s.
    Object(BTreeMap<String, Value>),
}

macro_rules! impl_is {
    ($name:ident, Value::$variant:ident) => {
        /// Checks if the value is the given type.
        pub fn $name(&self) -> bool {
            matches!(self, Value::$variant(_))
        }
    };
}

macro_rules! impl_as {
    ($name:ident, Value::$variant:ident, $type:ty) => {
        /// Returns the value as the given type if it is of that type.
        /// Otherwise, returns `None`.
        pub fn $name(&self) -> Option<&$type> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
    (mut, $name:ident, Value::$variant:ident, $type:ty) => {
        /// Returns the value as the given type if it is of that type.
        /// Otherwise, returns `None`.
        pub fn $name(&mut self) -> Option<&mut $type> {
            match self {
                Value::$variant(v) => Some(v),
                _ => None,
            }
        }
    };
}

impl Value {
    /// Checks if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    impl_is!(is_bool, Value::Bool);
    impl_is!(is_number, Value::Number);
    impl_is!(is_string, Value::String);
    impl_is!(is_array, Value::Array);
    impl_is!(is_object, Value::Object);
    impl_as!(as_bool, Value::Bool, bool);
    impl_as!(as_number, Value::Number, Number);
    impl_as!(as_string, Value::String, String);
    impl_as!(as_array, Value::Array, Vec<Value>);
    impl_as!(as_object, Value::Object, BTreeMap<String, Value>);
    impl_as!(mut, as_mut_bool, Value::Bool, bool);
    impl_as!(mut, as_mut_number, Value::Number, Number);
    impl_as!(mut, as_mut_string, Value::String, String);
    impl_as!(mut, as_mut_array, Value::Array, Vec<Value>);
    impl_as!(mut, as_mut_object, Value::Object, BTreeMap<String, Value>);
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn escape(s: &str) -> String {
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('/', "\\/")
                .replace('\u{0008}', "\\b")
                .replace('\u{000C}', "\\f")
                .replace('\u{000A}', "\\n")
                .replace('\u{000D}', "\\r")
                .replace('\u{0009}', "\\t")
        }

        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, r#""{}""#, escape(s)),
            Self::Array(a) => write!(
                f,
                "[{}]",
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Object(o) => write!(
                f,
                "{{{}}}",
                o.iter()
                    .map(|x| format!(r#""{}":{}"#, escape(x.0), x.1))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.parse()
    }
}
