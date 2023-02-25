use super::*;

pub type TokenChecker = fn(token: Token) -> bool;
pub fn space_token_checker(token: Token) -> bool {
    if let TokenT::Space(_) = token.t {
        true
    } else {
        false
    }
}

pub fn skip_spaces(token_stream: TokenStream) -> TokenStream {
    let mut token_stream = token_stream;

    let first = token_stream.first();
    if let Some(first) = first {
        if let TokenT::Space(_) = first.t.clone() {
            token_stream.remove(0);
            token_stream = skip_spaces(token_stream);
        }
    }

    let last = token_stream.last();
    if let Some(last) = last {
        if let TokenT::Space(_) = last.t.clone() {
            token_stream.pop();
            token_stream = skip_spaces(token_stream);
        }
    }

    token_stream
}

pub fn deliminated(
    token_stream: TokenStream,
    start: TokenT,
    end: TokenT,
) -> Option<(TokenStream, TokenStream)> {
    let mut token_stream = token_stream;

    let first = token_stream.first();
    if let Some(first) = first {
        if first.t == start {
            let mut deliminated = TokenStream::new();
            let mut rest = TokenStream::new();
            token_stream.remove(0);
            let mut count = 0;
            let mut ended = false;

            for token in token_stream.iter() {
                if ended {
                    rest.push(token.clone());
                } else {
                    if token.t == end {
                        if count == 0 {
                            ended = true;
                            continue;
                        } else {
                            count += 1;
                        }
                    }
                    deliminated.push(token.clone());
                }
            }

            return Some((deliminated, rest));
        }
    }

    None
}

pub fn split_stream(
    token_stream: TokenStream,
    checker: TokenChecker,
) -> (TokenStream, TokenStream) {
    let mut left = TokenStream::new();
    let mut right = TokenStream::new();

    let mut now = &mut left;

    for token in token_stream {
        if checker(token.clone()) {
            now = &mut right;
            continue;
        }
        now.push(token);
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_spaces_working() {
        let token_stream = tokenize("  hello_world\n\t".into());
        assert_eq!(
            skip_spaces(token_stream.clone()),
            vec![token_stream.get(2).unwrap().clone(),]
        )
    }

    #[test]
    fn deliminated_working() {
        let token_stream = tokenize("{hello}world".into());
        assert_eq!(
            deliminated(
                token_stream.clone(),
                TokenT::Bracket(Bracket::Open(BracketT::Curly)),
                TokenT::Bracket(Bracket::Close(BracketT::Curly)),
            ),
            Some((
                vec![token_stream.get(1).unwrap().clone()],
                vec![token_stream.get(3).unwrap().clone()]
            )),
        )
    }

    #[test]
    fn split_stream_working() {
        let token_stream = tokenize("hello/world".into());
        assert_eq!(
            split_stream(token_stream.clone(), |token| {
                token.t == TokenT::Operator(Operator::Slash)
            }),
            ((
                vec![token_stream.get(0).unwrap().clone()],
                vec![token_stream.get(2).unwrap().clone()]
            )),
        )
    }
}
