use std;

mod rust_9cc;

use rust_9cc::common::Integer;
use rust_9cc::common::Operator;

use rust_9cc::tokenizer::Tokenizer;
use rust_9cc::tokenizer::Token;
use rust_9cc::tokenizer::TokenKind;


fn main() {
    let arg = std::env::args().nth(1);
    let code = match arg {
        Some(code) => code,
        None => {
            panic!("Error: No argument is specified, abort.");
        }
    };

    let mut tokenizer = Tokenizer { expr: &code };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let token = tokenizer.next().unwrap();
    println!("  mov rax, {}", expect_number(token));
    loop {
        let token = match tokenizer.next() {
            Some(token) => token,
            None => break,
        };

        match token.kind {
            TokenKind::Reserved(operator) => match operator {
                Operator::Plus => {
                    let token = tokenizer.next().unwrap();
                    println!("  add rax, {}", expect_number(token));
                },
                Operator::Minus => {
                    let token = tokenizer.next().unwrap();
                    println!("  sub rax, {}", expect_number(token));
                },
            },
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    println!("  ret");
}

fn expect_number(token: Token) -> Integer {
    match token {
        Token { kind: TokenKind::Number(i), .. } => {
            return i;
        },
        _ => {
            panic!("Not a number: {}", token.expr);
        }
    }
}
