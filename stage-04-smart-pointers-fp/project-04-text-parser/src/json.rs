// 简化版 JSON 解析器
// 用手写 Parser 结构 + 字符级状态机

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Number(n) => write!(f, "{n}"),
            Json::String(s) => write!(f, "\"{}\"", s),
            Json::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Json::Object(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{k}\": {v}")?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// 解析器：维护 input 引用 + 当前 pos
pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    /// 顶层入口：跳空白 → 解析一个值 → 跳空白 → 校验无尾部
    pub fn parse(&mut self) -> Result<Json, String> {
        self.skip_whitespace();
        let v = self.parse_value()?;
        self.skip_whitespace();
        if self.pos < self.input.len() {
            return Err(format!("trailing chars at pos {}", self.pos));
        }
        Ok(v)
    }

    /// 跳 ASCII 空白
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    /// 偷看当前字符（不动 pos）
    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// 消耗并返回当前字符
    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    /// 必须消耗指定字符
    fn expect(&mut self, expected: char) -> Result<(), String> {
        let c = self.bump().ok_or_else(|| "unexpected end of input".to_string())?;
        if c != expected {
            return Err(format!("expected '{expected}', got '{c}'"));
        }
        Ok(())
    }

    /// 消耗字面量
    fn consume(&mut self, lit: &str) -> Result<(), String> {
        if self.input[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            Ok(())
        } else {
            Err(format!("expected literal '{lit}'"))
        }
    }

    /// 根据首字符分发
    fn parse_value(&mut self) -> Result<Json, String> {
        self.skip_whitespace();
        let c = self.peek().ok_or_else(|| "unexpected end of input".to_string())?;
        match c {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            other => Err(format!("unexpected character '{other}'")),
        }
    }

    fn parse_null(&mut self) -> Result<Json, String> {
        self.consume("null")?;
        Ok(Json::Null)
    }

    fn parse_bool(&mut self) -> Result<Json, String> {
        if self.input[self.pos..].starts_with("true") {
            self.pos += 4;
            Ok(Json::Bool(true))
        } else if self.input[self.pos..].starts_with("false") {
            self.pos += 5;
            Ok(Json::Bool(false))
        } else {
            Err("expected 'true' or 'false'".to_string())
        }
    }

    fn parse_string(&mut self) -> Result<Json, String> {
        self.expect('"')?;
        let start = self.pos;
        loop {
            match self.peek() {
                Some('"') => break,
                Some('\\') => {
                    // 简化：跳过转义符号 + 下一字符
                    self.pos += 1;
                    if self.peek().is_none() {
                        return Err("unterminated escape".to_string());
                    }
                    self.pos += self.peek().unwrap().len_utf8();
                }
                Some(c) => self.pos += c.len_utf8(),
                None => return Err("unterminated string".to_string()),
            }
        }
        let s = self.input[start..self.pos].to_string();
        self.expect('"')?;
        Ok(Json::String(s))
    }

    fn parse_number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.pos += 1;
        }
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let s = &self.input[start..self.pos];
        s.parse::<f64>()
            .map(Json::Number)
            .map_err(|e| format!("invalid number '{s}': {e}"))
    }

    fn parse_array(&mut self) -> Result<Json, String> {
        self.expect('[')?;
        let mut items = vec![];
        self.skip_whitespace();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Json::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(',') => { self.pos += 1; }
                Some(']') => { self.pos += 1; return Ok(Json::Array(items)); }
                _ => return Err("expected ',' or ']'".to_string()),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Json, String> {
        self.expect('{')?;
        let mut entries = vec![];
        self.skip_whitespace();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Json::Object(entries));
        }
        loop {
            self.skip_whitespace();
            let key = match self.parse_value()? {
                Json::String(s) => s,
                _ => return Err("expected string key".to_string()),
            };
            self.skip_whitespace();
            self.expect(':')?;
            let value = self.parse_value()?;
            entries.push((key, value));
            self.skip_whitespace();
            match self.peek() {
                Some(',') => { self.pos += 1; }
                Some('}') => { self.pos += 1; return Ok(Json::Object(entries)); }
                _ => return Err("expected ',' or '}'".to_string()),
            }
        }
    }
}

/// 顶层便捷函数
pub fn parse(input: &str) -> Result<Json, String> {
    Parser::new(input).parse()
}
