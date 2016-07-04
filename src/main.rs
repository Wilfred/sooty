/// A value in our little language. For now, just an integer.
#[derive(Debug)]
struct Value {
    value: i64,
}

fn lex(source: &str) -> Vec<Value> {
    vec![Value { value: 42 }]
}

fn main() {
    loop {
        println!("sooty> ");

        println!("{:?}", lex("42"));
        break;
    }
}
