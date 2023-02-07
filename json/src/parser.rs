use std::collections::BTreeMap;

use crate::{error::JsonResult, Error, Value};

pub(super) struct Parser<'a> {
    input: &'a str,
    len: usize,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub(super) fn new(inp: &'a str) -> Self {
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

    pub(super) fn parse(&mut self) -> Result<Value, Error> {
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
        while self.pos < self.len && is_digit(self.next()) {}

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
