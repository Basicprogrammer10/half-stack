//! A JSON parser and serializer.

use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

type JsonResult<T> = Result<T, Error>;

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

struct Parser<'a> {
    input: &'a str,
    len: usize,
    pos: usize,
}

/// Errors that can occur during parsing.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// An unexpected character was encountered at the given position.
    UnexpectedChar(usize),
    ///An invalid number was encountered.
    InvalidNumber(ParseNumberError),
    /// The end of the input was reached unexpectedly.
    UnexpectedEnd(usize),
    /// An invalid escape sequence was encountered.
    InvalidEscape(char),
}

/// Errors that can occur during parsing of a number.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseNumberError {
    /// An error occurred while parsing an integer.
    ParseIntError(ParseIntError),
    /// An error occurred while parsing a float.
    ParseFloatError(ParseFloatError),
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
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
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

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UInt(x) => write!(f, "{}", x),
            Self::Int(x) => write!(f, "{}", x),
            Self::Float(x) => write!(f, "{}", x),
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

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);
        parser.parse()
    }
}

impl<'a> Parser<'a> {
    fn new(inp: &'a str) -> Self {
        Self {
            input: inp,
            len: inp.len(),
            pos: 0,
        }
    }

    fn char(&self, pos: usize) -> char {
        self.input.as_bytes()[pos] as char
    }

    fn next(&mut self) -> char {
        self.pos += 1;
        self.char(self.pos - 1)
    }

    fn require_chars(&mut self, chars: &[u8]) -> JsonResult<()> {
        for i in chars {
            if self.pos >= self.len {
                return Err(Error::UnexpectedEnd(self.pos));
            }

            if self.next() != *i as char {
                return Err(Error::UnexpectedChar(self.pos));
            }
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<Value, Error> {
        if self.len == 0 {
            return Err(Error::UnexpectedEnd(self.pos));
        }

        self.skip_whitespace();
        let chr = self.char(self.pos);
        match chr {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '0'..='9' | '-' => self.parse_number(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            x => todo!("Error {x}"),
        }
    }

    fn skip_whitespace(&mut self) {
        fn skip(x: char) -> bool {
            x.is_whitespace() || x == ','
        }

        while self.pos < self.len && skip(self.next()) {}
        self.pos -= 1;
    }

    fn parse_null(&mut self) -> JsonResult<Value> {
        self.require_chars(b"null")?;
        Ok(Value::Null)
    }

    fn parse_bool(&mut self) -> JsonResult<Value> {
        let expected = self.next();
        match expected {
            't' => self.require_chars(b"rue")?,
            'f' => self.require_chars(b"alse")?,
            _ => return Err(Error::UnexpectedChar(self.pos)),
        };

        Ok(Value::Bool(expected == 't'))
    }

    fn parse_number(&mut self) -> JsonResult<Value> {
        fn is_digit(digit: char) -> bool {
            ('0'..='9').contains(&digit) || matches!(digit, '-' | '.')
        }

        let start = self.pos;
        while self.pos < self.len && is_digit(self.next() as char) {}

        let num = &self.input[start..self.pos];
        Ok(Value::Number(num.parse()?))
    }

    fn parse_string(&mut self) -> JsonResult<Value> {
        fn unescape(s: &str) -> Result<String, Error> {
            let mut out = String::new();
            let mut escape = false;

            for i in s.chars() {
                if escape {
                    match i {
                        '"' => out.push('"'),
                        '\\' => out.push('\\'),
                        '/' => out.push('/'),
                        'b' => out.push('\x08'),
                        'f' => out.push('\x0C'),
                        'n' => out.push('\x0A'),
                        'r' => out.push('\x0D'),
                        't' => out.push('\x09'),
                        _ => return Err(Error::InvalidEscape(i)),
                    }
                    escape = false;
                    continue;
                }

                if i == '\\' {
                    escape = true;
                    continue;
                }

                out.push(i);
            }

            Ok(out)
        }

        self.pos += 1;
        let start = self.pos;
        let mut escape = false;
        while self.pos < self.len && (self.char(self.pos) != '"' || escape) {
            if self.char(self.pos) == '\\' {
                escape = true;
                self.pos += 1;
                continue;
            }
            self.pos += 1;
            escape = false;
        }

        if self.pos == self.len && self.char(self.pos - 1) != '"' {
            return Err(Error::UnexpectedEnd(self.pos));
        }

        let string = &self.input[start..self.pos];
        self.pos += 1;
        Ok(Value::String(unescape(string)?))
    }

    fn parse_array(&mut self) -> JsonResult<Value> {
        self.pos += 1;
        let start = self.pos;
        let mut depth = 1;
        while self.pos < self.len && depth != 0 {
            match self.next() {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {}
            }
        }

        if self.pos == self.len && depth != 0 {
            return Err(Error::UnexpectedEnd(self.pos));
        }

        let end = self.pos;
        self.pos = start;

        let mut tokens = Vec::new();
        while self.pos < end.saturating_sub(1) {
            self.skip_whitespace();
            tokens.push(self.parse()?);
        }

        self.pos += 1;
        Ok(Value::Array(tokens))
    }

    fn parse_object(&mut self) -> JsonResult<Value> {
        self.pos += 1;
        let start = self.pos;
        let mut depth = 1;
        while self.pos < self.len && depth != 0 {
            match self.next() {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
        }

        if self.pos == self.len && depth != 0 {
            return Err(Error::UnexpectedEnd(self.pos));
        }

        let end = self.pos;
        self.pos = start;

        let mut tokens = BTreeMap::new();
        while self.pos < end.saturating_sub(1) {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.require_chars(b":")?;
            self.skip_whitespace();
            let value = self.parse()?;

            let name = match key {
                Value::String(s) => s,
                _ => unreachable!(),
            };
            tokens.insert(name, value);
        }

        self.pos += 1;
        Ok(Value::Object(tokens))
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::InvalidNumber(ParseNumberError::ParseIntError(e))
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Error::InvalidNumber(ParseNumberError::ParseFloatError(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let mut parser = Parser::new("");
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(0)));
    }

    #[test]
    fn test_null() {
        let mut parser = Parser::new("null");
        assert_eq!(parser.parse(), Ok(Value::Null));
    }

    #[test]
    fn test_null_fail() {
        let mut parser = Parser::new("nul");
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(3)));
    }

    #[test]
    fn test_bool() {
        let mut parser = Parser::new("true");
        assert_eq!(parser.parse(), Ok(Value::Bool(true)));

        let mut parser = Parser::new("false");
        assert_eq!(parser.parse(), Ok(Value::Bool(false)));
    }

    #[test]
    fn test_bool_fail() {
        let mut parser = Parser::new("tru");
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(3)));

        let mut parser = Parser::new("fals");
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(4)));
    }

    #[test]
    fn test_number() {
        let mut parser = Parser::new("123");
        assert_eq!(parser.parse(), Ok(Value::Number(Number::UInt(123))));

        let mut parser = Parser::new("-123");
        assert_eq!(parser.parse(), Ok(Value::Number(Number::Int(-123))));

        let mut parser = Parser::new("123.456");
        assert_eq!(parser.parse(), Ok(Value::Number(Number::Float(123.456))));
    }

    #[test]
    fn test_number_fail() {
        let mut parser = Parser::new("123d");
        assert!(matches!(parser.parse(), Err(Error::InvalidNumber(_))));

        let mut parser = Parser::new("123.456.789");
        assert!(matches!(parser.parse(), Err(Error::InvalidNumber(_))));
    }

    #[test]
    fn test_string() {
        let mut parser = Parser::new(r#""hello""#);
        assert_eq!(parser.parse(), Ok(Value::String("hello".to_string())));
    }

    #[test]
    fn test_string_fail() {
        let mut parser = Parser::new(r#""hello"#);
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(6)));
    }

    #[test]
    fn test_array() {
        let mut parser = Parser::new(r#"["hello", "world"]"#);
        assert_eq!(
            parser.parse(),
            Ok(Value::Array(vec![
                Value::String("hello".to_string()),
                Value::String("world".to_string())
            ]))
        );
    }

    #[test]
    fn test_array_fail() {
        let mut parser = Parser::new(r#"["hello", "world""#);
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(17)));
    }

    #[test]
    fn test_nested_array() {
        let mut parser = Parser::new(r#"[["hello", "world"], ["hello", "world"]]"#);
        assert_eq!(
            parser.parse(),
            Ok(Value::Array(vec![
                Value::Array(vec![
                    Value::String("hello".to_string()),
                    Value::String("world".to_string())
                ]),
                Value::Array(vec![
                    Value::String("hello".to_string()),
                    Value::String("world".to_string())
                ])
            ]))
        );
    }

    #[test]
    fn test_nested_array_fail() {
        let mut parser = Parser::new(r#"[["hello", "world"], ["hello", "world"]"#);
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(39)));
    }

    #[test]
    fn test_object() {
        let mut parser = Parser::new(r#"{"hello": "world"}"#);
        let mut map = BTreeMap::new();
        map.insert("hello".to_string(), Value::String("world".to_string()));
        assert_eq!(parser.parse(), Ok(Value::Object(map)));
    }

    #[test]
    fn test_object_fail() {
        let mut parser = Parser::new(r#"{"hello": "world""#);
        assert_eq!(parser.parse(), Err(Error::UnexpectedEnd(17)));
    }

    #[test]
    fn test_string_escape() {
        let value = Value::from_str(r#""hello \"world\""#).unwrap();
        assert_eq!(value.as_string().unwrap(), "hello \"world\"");

        let value = Value::from_str(r#"{"hello":"\"world\""}"#).unwrap();
        assert_eq!(
            value
                .as_object()
                .unwrap()
                .get("hello")
                .unwrap()
                .as_string()
                .unwrap(),
            "\"world\""
        );
    }

    #[test]
    fn test_api() {
        let value = Value::from_str(r#"{"hello": "world"}"#).unwrap();
        assert_eq!(
            value
                .as_object()
                .unwrap()
                .get("hello")
                .unwrap()
                .as_string()
                .unwrap(),
            "world"
        );
    }

    #[test]
    fn test_to_string() {
        let value = Value::from_str(r#"{"hello": "world"}"#).unwrap();
        assert_eq!(value.to_string(), r#"{"hello":"world"}"#);

        let value = Value::from_str(r#"{"hello": "world", "foo": "bar"}"#).unwrap();
        assert_eq!(value.to_string(), r#"{"foo":"bar","hello":"world"}"#);

        let value = Value::from_str(r#"[{"hello": "world"}, {"foo": "bar"}]"#).unwrap();
        assert_eq!(value.to_string(), r#"[{"hello":"world"},{"foo":"bar"}]"#);
    }
}
