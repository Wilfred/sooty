use self::Lexeme::*;

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    LeftParen,
    RightParen,
    Integer {
        value: i64,
    },
}

pub fn lex(source: &str) -> Result<Vec<Lexeme>, ()> {
    let mut result = vec![];
    for part in source.split_whitespace() {
        let lexeme = match part {
            "(" => LeftParen,
            ")" => RightParen,
            _ => {
                match part.parse::<i64>() {
                    Ok(num) => Integer { value: num },
                    Err(..) => {
                        return Err(());
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
