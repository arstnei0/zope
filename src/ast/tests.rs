use super::*;

fn fail() {
    assert!(false);
}

#[test]
fn parse_identifier_expr() {
    let mut parser = Parser::new("foo bar");
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Identifier(IdentifierExpr::new("foo".to_string()))
    );
    parser.lexer.ignore_spaces();
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Identifier(IdentifierExpr::new("bar".to_string()))
    );
}

#[test]
fn parse_literal_expr() {
    // Number
    let mut parser = Parser::new("42 23");
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Literal(LiteralExpr::Number(42))
    );
    parser.lexer.ignore_spaces();
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Literal(LiteralExpr::Number(23))
    );

    // Bool
    parser.reload("true false");
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Literal(LiteralExpr::Bool(true))
    );
    parser.lexer.ignore_spaces();
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Literal(LiteralExpr::Bool(false))
    );

    // String
    parser.reload("'hello'");
    assert_eq!(
        parser.parse_expr().unwrap().kind,
        ExprKind::Literal(LiteralExpr::String("hello".to_string()))
    );
}

#[test]
fn parse_call_expr() {
    let mut parser = Parser::new("foo(bar) baz(input) add(1)");
    let call = parser.parse_expr().unwrap().kind;
    if let ExprKind::Call(call) = call {
        if let ExprKind::Identifier(identifier) = call.called.kind {
            assert_eq!(identifier.ident, "foo");
        } else {
            fail()
        }
        if let ExprKind::Identifier(identifier) = call.input.kind {
            assert_eq!(identifier.ident, "bar");
        } else {
            fail()
        }
    } else {
        fail()
    }

    parser.lexer.ignore_spaces();
    let call = parser.parse_expr().unwrap().kind;
    if let ExprKind::Call(call) = call {
        if let ExprKind::Identifier(identifier) = call.called.kind {
            assert_eq!(identifier.ident, "baz");
        } else {
            fail()
        }
        if let ExprKind::Identifier(identifier) = call.input.kind {
            assert_eq!(identifier.ident, "input");
        } else {
            fail()
        }
    } else {
        fail()
    }

    parser.lexer.ignore_spaces();
    let call = parser.parse_expr().unwrap().kind;
    if let ExprKind::Call(call) = call {
        if let ExprKind::Identifier(identifier) = call.called.kind {
            assert_eq!(identifier.ident, "add");
        } else {
            fail()
        }
        assert_eq!(call.input.kind, ExprKind::Literal(LiteralExpr::Number(1)));
    } else {
        fail()
    }
}

#[test]
fn parse_let_stmt() {
    let mut parser = Parser::new("let foo = bar(baz);");
    let stmt = parser.parse_stmt().unwrap();

    assert_eq!(
        stmt,
        Stmt::new(
            StmtKind::Let(LetStmt::new(
                "foo".into(),
                Expr::new(
                    ExprKind::Call(CallExpr::new(
                        Expr::new(
                            ExprKind::Identifier(IdentifierExpr::new("bar".to_string())),
                            Position::new(10, 12)
                        ),
                        Expr::new(
                            ExprKind::Identifier(IdentifierExpr::new("baz".to_string())),
                            Position::new(14, 16)
                        )
                    )),
                    Position::new(10, 17)
                )
            )),
            Position::new(0, 18)
        )
    );
}
