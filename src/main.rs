use std;

mod rust_9cc;

fn main() {
    let arg = std::env::args().nth(1);
    let code = match arg {
        Some(code) => code,
        None => {
            panic!("Error: No argument is specified, abort.");
        }
    };

    let mut tokenizer = rust_9cc::Tokenizer::from(code.as_str());

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
            rust_9cc::TokenKind::Reserved(operator) => match operator {
                rust_9cc::Operator::Plus => {
                    let token = tokenizer.next().unwrap();
                    println!("  add rax, {}", expect_number(token));
                }
                rust_9cc::Operator::Minus => {
                    let token = tokenizer.next().unwrap();
                    println!("  sub rax, {}", expect_number(token));
                }
            },
            _ => rust_9cc::print_error_pos(token.code, token.pos, "Error: cannot parse"),
        }
    }

    println!("  ret");
}

fn expect_number(token: rust_9cc::Token) -> rust_9cc::Integer {
    match token.kind {
        rust_9cc::TokenKind::Number(i) => {
            return i;
        }
        _ => rust_9cc::print_error_pos(token.code, token.pos, "Not a number"),
    }
}
