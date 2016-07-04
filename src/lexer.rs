// TODO: move Value to a separate file.

/// A value in our little language. For now, just an integer.
#[derive(Debug, PartialEq)]
pub struct Value {
    value: i64,
}

pub fn lex(source: &str) -> Result<Vec<Value>, ()> {
    let mut result = vec![];
    for part in source.split_whitespace() {
        match part.parse::<i64>() {
            Ok(num) => {
                result.push(Value { value: num });
            }
            Err(..) => {
                return Err(());
            }
        }
    }
    Ok(result)
}

#[test]
fn test_lex_single_number() {
    let lexed = lex(&"41").unwrap();
    assert!(lexed == vec![Value { value: 41 }]);
}

#[test]
fn test_lex_multiple_numbers() {
    let lexed = lex(&"42 42").unwrap();
    assert!(lexed == vec![Value { value: 42 }, Value { value: 42 }]);
}
