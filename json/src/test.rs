use std::{collections::BTreeMap, str::FromStr};

use crate::parser::Parser;

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
