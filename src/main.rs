use std::io;

/// A value in our little language. For now, just an integer.
#[derive(Debug)]
struct Value {
    value: i64,
}

fn lex(source: &str) -> Vec<Value> {
    vec![Value { value: 42 }]
}

fn read() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn repl() {
    loop {
        println!("sooty> ");

        let input = read();
        println!("{:?}", lex(&input));
        break;
    }
}

fn main() {
    repl();
}
