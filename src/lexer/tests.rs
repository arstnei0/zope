use crate::lexer::{bracket::Bracket, position::Position, *};

#[test]
fn parse_bracket() {
    let test_str = "({[<>]}))";
    let mut lexer = Lexer::new(test_str.chars());
    let token = lexer.bump().unwrap();
    assert_eq!(
        token,
        Token {
            pos: Position { start: 0, end: 0 },
            kind: TokenKind::Bracket(Bracket::OpenParen)
        }
    );
    let char = token.pos.get(test_str);
    assert_eq!(char, Some("(".to_string()));
}

#[test]
fn parse_keyword() {
    let test_str = "let if";
    let mut lexer = Lexer::new(test_str.chars());
    let token = lexer.bump().unwrap();
    assert_eq!(
        token,
        Token {
            pos: Position { start: 0, end: 2 },
            kind: TokenKind::Keyword(Keyword::Let)
        }
    );
    let char = token.pos.get(test_str);
    assert_eq!(char, Some("let".to_string()));

    lexer.bump();

    let token = lexer.bump().unwrap();
    assert_eq!(
        token,
        Token {
            pos: Position { start: 4, end: 5 },
            kind: TokenKind::Keyword(Keyword::If)
        }
    );
    let char = token.pos.get(test_str);
    assert_eq!(char, Some("if".to_string()));
}

#[test]
fn parse_separation() {
    let test_str = "'hello world'";
    let mut lexer = Lexer::new(test_str.chars());
    let token = lexer.bump().unwrap();
    assert_eq!(
        token,
        Token {
            pos: Position { start: 0, end: 12 },
            kind: TokenKind::Separation(Separation::new(
                Separator::SingleQuote,
                "hello world".to_string()
            ))
        }
    );
    let char = token.pos.get(test_str);
    assert_eq!(char, Some("'hello world'".to_string()));
}
