#[derive(Debug, PartialEq)]
pub enum Lexeme {
    LeftParen,
    RightParen,
    Integer {
        value: i64,
    },
}

/// A OwningValue where all lists own their subelements. Easy to use safely,
/// but forbids sharing list subelements.
#[derive(Debug)]
pub enum OwningValue {
    Integer {
        value: i64,
    },
    List {
        items: Vec<Box<OwningValue>>,
    },
}

pub fn lex(source: &str) -> Result<Vec<Lexeme>, String> {
    let mut result = vec![];

    for mut part in source.split_whitespace() {
        if part.starts_with("(") {
            result.push(Lexeme::LeftParen);

            if part == "(" {
                continue;
            } else {
                part = &part[1..];
            }
        }

        if part.starts_with(")") {
            result.push(Lexeme::RightParen);

            if part == ")" {
                continue;
            } else {
                part = &part[1..];
            }
        }

        match part.parse::<i64>() {
            Ok(num) => {
                result.push(Lexeme::Integer { value: num });
            }
            Err(..) => {
                return Err(format!("Could not lex {}", part));
            }
        }
    }
    Ok(result)
}

pub fn parse(lexemes: Vec<Lexeme>) -> Result<Vec<OwningValue>, String> {
    let mut stack: Vec<_> = vec![];

    let root_list = OwningValue::List { items: vec![] };
    stack.push(root_list);

    for lexeme in lexemes {
        match lexeme {
            Lexeme::Integer { value } => {
                let int_value = OwningValue::Integer { value: value };

                match stack.last_mut() {
                    Some(current_list) => {
                        match current_list {
                            &mut OwningValue::List { ref mut items } => {
                                items.push(Box::new(int_value))
                            }
                            _ => {
                                panic!("Need a list to append a number to!");
                            }
                        }
                    }
                    None => {
                        panic!("Trying to append a number to an empty stack!");
                    }
                }
            }
            Lexeme::LeftParen => {
                // Starting a new list.
                let new_list = OwningValue::List { items: vec![] };
                stack.push(new_list);
            }
            Lexeme::RightParen => {
                let current_list = stack.pop().expect("Empty stack: Need a list to terminate");
                match stack.last_mut() {
                    Some(mut parent_list) => {
                        match parent_list {
                            &mut OwningValue::List { ref mut items } => {
                                items.push(Box::new(current_list));
                            }
                            _ => {
                                panic!("Need a list to append this list to");
                            }
                        }
                    }
                    _ => {
                        return Err("Unmatched ) paren.".to_owned());
                    }
                }
            }
        }
    }

    let result = stack.pop().expect("Needed a value from the parser to return");

    if !stack.is_empty() {
        return Err("Parsing had data left over: unclosed ( paren".to_owned());
    }
    match result {
        OwningValue::List {items} => {
            return Ok(items.into_iter().map(|boxed_item| *boxed_item).collect());
        }
        _ => {
            panic!("Final parsed value was not a list")
        }
    }
}

#[test]
fn test_lex_single_number() {
    let lexed = lex(&"41").unwrap();
    assert!(lexed == vec![Lexeme::Integer { value: 41 }]);
}

#[test]
fn test_lex_multiple_numbers() {
    let lexed = lex(&"42 42").unwrap();
    assert!(lexed == vec![Lexeme::Integer { value: 42 }, Lexeme::Integer { value: 42 }]);
}

#[test]
fn test_lex_left_paren() {
    let lexed = lex(&"(").unwrap();
    assert!(lexed == vec![Lexeme::LeftParen]);
}

#[test]
fn test_lex_left_paren_and_number() {
    let lexed = lex(&"(1").unwrap();
    assert!(lexed == vec![Lexeme::LeftParen, Lexeme::Integer { value: 1 }]);
}

#[test]
fn test_lex_right_paren() {
    let lexed = lex(&")").unwrap();
    assert!(lexed == vec![Lexeme::RightParen]);
}


#[test]
fn test_parse_unbalanced_left() {
    let lexed = lex(&"(").unwrap();
    let parsed = parse(lexed);
    assert!(parsed.is_err());
}

#[test]
fn test_parse_unbalanced_right() {
    let lexed = lex(&")").unwrap();
    let parsed = parse(lexed);
    assert!(parsed.is_err());
}

#[test]
fn test_parse_empty() {
    let lexed = lex(&"()").unwrap();
    let parsed = parse(lexed);
    assert!(parsed.is_ok());
}
