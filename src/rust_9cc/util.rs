pub fn print_error_pos(code: &str, pos: usize, msg: &str) -> ! {
    eprintln!("{}", code);
    eprint!("{}", (0..pos).map(|_| " ").collect::<String>());
    eprintln!("^");
    eprintln!("{}", msg);
    std::process::exit(1);
}
