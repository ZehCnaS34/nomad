#[derive(Debug, PartialEq)]
enum Error {
    Tokenize,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Quote,
}

#[derive(Debug, PartialEq)]
struct Token {
    kind: TokenKind,
    text: String,
    offset: usize,
}

fn tokenize(source: &str) -> Result<Vec<Token>, Error> {
    let mut source = source.chars().into_iter().enumerate().peekable();
    let mut tokens = vec![];
    let mut token = String::new();

    while let Some(&(i, c)) = source.peek() {
        match c {
            '(' => {
                tokens.push(Token {
                    kind: TokenKind::OpenParen,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            ')' => {
                tokens.push(Token {
                    kind: TokenKind::CloseParen,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            '[' => {
                tokens.push(Token {
                    kind: TokenKind::OpenBracket,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            ']' => {
                tokens.push(Token {
                    kind: TokenKind::CloseBracket,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            '\'' => {
                tokens.push(Token {
                    kind: TokenKind::Quote,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            _ => {
                panic!("Awesome");
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        simple_tokens_helper(TokenKind::OpenParen, "(");
        simple_tokens_helper(TokenKind::CloseParen, ")");
        simple_tokens_helper(TokenKind::OpenBracket, "[");
        simple_tokens_helper(TokenKind::CloseBracket, "]");
        simple_tokens_helper(TokenKind::Quote, "'");
    }

    fn simple_tokens_helper(kind: TokenKind, src: &str) {
        assert_eq!(
            Ok(vec![Token {
                kind,
                text: String::from(src),
                offset: 0
            }]),
            tokenize(src)
        )
    }
}
