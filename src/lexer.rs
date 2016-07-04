// TODO: move Value to a separate file.

/// A value in our little language. For now, just an integer.
#[derive(Debug, PartialEq)]
pub struct Value {
    value: i64,
}

pub fn lex(source: &str) -> Vec<Value> {
    let mut result = vec![];
    for part in source.split_whitespace() {
        result.push(Value { value: 42 });
    }
    result
}

#[test]
fn test_lex_multiple_numbers() {
    let lexed = lex(&"42 42");
    assert!(lexed == vec![Value { value: 42}, Value { value: 42}]);
}
