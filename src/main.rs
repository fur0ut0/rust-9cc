use std;

type Integer = i32;

fn main() {
    let arg = std::env::args().nth(1);
    let code = match arg {
        Some(code) => code,
        None => {
            panic!("Error: No argument is specified, abort.");
        }
    };

    let mut remained = code.as_str();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let integer;
    (integer, remained) = consume_number(remained).unwrap();
    println!("  mov rax, {}", integer);
    while !remained.is_empty() {
        let (head, tail) = consume_reserved(remained).unwrap();
        match head {
            "+" => {
                let integer;
                (integer, remained) = consume_number(tail).unwrap();
                println!("  add rax, {}", integer);
            }
            "-" => {
                let integer;
                (integer, remained) = consume_number(tail).unwrap();
                println!("  sub rax, {}", integer);
            }
            _ => (),
        }
    }

    println!("  ret");
}

fn consume_reserved(expr: &str) -> Option<(&str, &str)> {
    static TOKENS: &'static [&str] = &["+", "-"];
    for token in TOKENS {
        if expr.starts_with(token) {
            return Some(expr.split_at(token.len()));
        }
    }
    None
}

fn consume_number(expr: &str) -> Option<(Integer, &str)> {
    let index = expr.bytes().position(|b| !b.is_ascii_digit()).unwrap_or(expr.len());
    if index == 0 {
        return None
    }
    Some(expr.split_at(index)).map(|(head, tail)| (to_integer(head), tail))
}

fn to_integer(expr: &str) -> Integer {
    match expr.parse::<Integer>() {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: Not integer expression: {expr} ({e})");
        }
    }
}
