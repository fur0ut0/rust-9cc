use crate::rust_9cc::common::{Integer, Operator};

#[derive(Default)]
pub struct Tokenizer<'a> {
    code: &'a str,
    pos: usize,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(code: &'a str) -> Tokenizer {
        Self { code: code, pos: 0 }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut remain;
        (_, remain) = self.code.split_at(self.pos);

        let index = remain.bytes().position(|b| !b.is_ascii_whitespace())?;
        (_, remain) = remain.split_at(index);
        self.pos += index;

        if remain.len() == 0 {
            return None;
        }

        if let Some((operator, head, _)) = consume_reserved(remain) {
            self.pos += head.len();
            let kind = TokenKind::Reserved(operator);
            return Some(Token { kind, expr: head });
        }

        if let Some((integer, head, _)) = consume_number(remain) {
            self.pos += head.len();
            let kind = TokenKind::Number(integer);
            return Some(Token { kind, expr: head });
        }

        panic!("Error: cannot tokenize: {}", self.expr);
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub expr: &'a str,
}

#[derive(Debug)]
pub enum TokenKind {
    Reserved(Operator),
    Number(Integer),
}

fn expr_to_operator(expr: &str) -> Operator {
    match expr {
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        _ => {
            unreachable!("Not a valid expression for operator: {}", expr);
        }
    }
}

fn consume_reserved(expr: &str) -> Option<(Operator, &str, &str)> {
    static TOKENS: &'static [&str] = &["+", "-"];
    for token in TOKENS {
        if expr.starts_with(token) {
            let (head, tail) = expr.split_at(token.len());
            let operator = expr_to_operator(head);
            return Some((operator, head, tail));
        }
    }
    None
}

fn consume_number(expr: &str) -> Option<(Integer, &str, &str)> {
    let index = expr
        .bytes()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(expr.len());
    if index == 0 {
        return None;
    }
    let (head, tail) = expr.split_at(index);
    let integer = to_integer(head);
    Some((integer, head, tail))
}

fn to_integer(expr: &str) -> Integer {
    match expr.parse::<Integer>() {
        Ok(v) => v,
        Err(e) => {
            unreachable!("Error: Not integer expression: {expr} ({e})");
        }
    }
}
