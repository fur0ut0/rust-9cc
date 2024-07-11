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
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", to_integer(&code));
    println!("  ret");
}

fn to_integer(expr: &str) -> Integer {
    match expr.parse::<Integer>() {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: Not integer expression: {expr} ({e})");
        }
    }
}
