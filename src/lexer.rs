// TODO: move Value to a separate file.

/// A value in our little language. For now, just an integer.
#[derive(Debug)]
pub struct Value {
    value: i64,
}

pub fn lex(source: &str) -> Vec<Value> {
    vec![Value { value: 42 }]
}
