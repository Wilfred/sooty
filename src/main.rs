use std::io;
use std::io::Write;

mod lexer;

fn read() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn repl() {
    loop {
        print!("sooty> ");
        io::stdout().flush().unwrap();

        let input = read();

        if input.trim() == "(quit)" {
            break;
        }

        println!("{:?}", lexer::lex(&input));
    }
}

fn main() {
    println!("Sooty, a simple lisp.");
    println!("Press Ctrl-C or type (quit) to exit.\n");

    repl();
}
