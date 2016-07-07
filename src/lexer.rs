
#[derive(Debug, PartialEq)]
pub enum Lexeme {
    LeftParen,
    RightParen,
    Integer {
        value: i64,
    },
}

pub fn lex(source: &str) -> Result<Vec<Lexeme>, String> {
    let mut result = vec![];
    for part in source.split_whitespace() {
        let lexeme = match part {
            "(" => Lexeme::LeftParen,
            ")" => Lexeme::RightParen,
            _ => {
                match part.parse::<i64>() {
                    Ok(num) => Lexeme::Integer { value: num },
                    Err(..) => {
                        return Err(format!("Could not lex {}", part));
                    }
                }
            }
        };
        result.push(lexeme);
    }
    Ok(result)
}

#[test]
fn test_lex_single_number() {
    let lexed = lex(&"41").unwrap();
    assert!(lexed == vec![Integer { value: 41 }]);
}

#[test]
fn test_lex_multiple_numbers() {
    let lexed = lex(&"42 42").unwrap();
    assert!(lexed == vec![Integer { value: 42 }, Integer { value: 42 }]);
}

#[test]
fn test_lex_left_paren() {
    let lexed = lex(&"(").unwrap();
    assert!(lexed == vec![LeftParen]);
}

#[test]
fn test_lex_right_paren() {
    let lexed = lex(&")").unwrap();
    assert!(lexed == vec![RightParen]);
}
