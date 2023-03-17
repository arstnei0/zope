use super::*;

#[test]
fn parse_identifier() {
    let mut parser = Parser::from_str("foo bar");
    assert_eq!(
        parser.bump_next().unwrap().kind,
        ExprKind::Identifier(Identifier::new("foo".to_string()))
    );
    parser.bump_next();
    assert_eq!(
        parser.bump_next().unwrap().kind,
        ExprKind::Identifier(Identifier::new("bar".to_string()))
    );
}

#[test]
fn parse_call() {
    let mut parser = Parser::from_str("foo(bar)");
    let call = parser.bump_next().unwrap().kind;
    // println!("{:?}", call);
    // if let ExprKind::Call(call) = call {
    //     if let ExprKind::Identifier(identifier) = call.called.kind {
    //         assert_eq!(identifier.ident, "foo");
    //     } else {
    //         panic!()
    //     }
    //     if let ExprKind::Identifier(identifier) = call.input.kind {
    //         assert_eq!(identifier.ident, "bar");
    //     } else {
    //         panic!()
    //     }
    // } else {
    //     panic!()
    // }
}
