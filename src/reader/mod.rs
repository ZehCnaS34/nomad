#[derive(Debug, PartialEq)]
enum Error {}

#[derive(Debug, PartialEq)]
enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    BackQuote,
    Quote,
    Hash,
    Percent,
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
            '`' => {
                tokens.push(Token {
                    kind: TokenKind::BackQuote,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            '#' => {
                tokens.push(Token {
                    kind: TokenKind::Hash,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            '%' => {
                tokens.push(Token {
                    kind: TokenKind::Percent,
                    offset: i,
                    text: format!("{}", c),
                });
                source.next();
            }
            _ => {
                todo!();
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
        simple_tokens_helper(TokenKind::BackQuote, "`");
        simple_tokens_helper(TokenKind::Quote, "'");
        simple_tokens_helper(TokenKind::Hash, "#");
    }

    #[test]
    fn test_compound_token() {
        simple_tokens_helper(TokenKind::Percent, "%");
    }

    fn simple_tokens_helper(kind: TokenKind, src: &str) {
        let result = tokenize(src);
        assert_eq!(
            Ok(vec![Token {
                kind,
                text: String::from(src),
                offset: 0
            }]),
            result
        )
    }
}
