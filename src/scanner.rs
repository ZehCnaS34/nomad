use crate::context::Context;
use crate::result::{Issue, IssueType, NResult, Skimmer};
use crate::token::{Token, TokenType};
use crate::util::{
    is_colon, is_digit, is_dot, is_double_quote, is_left_brace, is_left_bracket, is_left_paren,
    is_newline, is_slash, is_symbol_char, is_symbol_start, is_whitespace,
};
use crate::view::{Cursor, View};

type CharView = View<char>;

impl Skimmer for CharView {
    type Item = Token;
    fn skim_issue(&self, issue: IssueType) -> NResult<Token> {
        let position = self.cursor.borrow();
        if let Some(lexeme) = self.data.get(position.start..position.current) {
            Err(Issue {
                issue,
                context: lexeme.iter().collect(),
                position: Some(position.clone()),
            })
        } else {
            Err(Issue {
                issue,
                context: "".into(),
                position: Some(position.clone()),
            })
        }
    }

    fn skim_token(&self, token_type: TokenType) -> NResult<Token> {
        let position = self.cursor.borrow();
        let lexeme = self.view();
        if lexeme.len() == 0 && token_type != TokenType::Eof {
            self.skim_issue(IssueType::ScanError)
        } else {
            Ok(Token {
                token_type,
                lexeme: lexeme.iter().collect(),
                position: position.clone(),
            })
        }
    }
}

fn scan_symbol(view: &CharView) -> NResult<Token> {
    fn inner(view: &CharView) {
        while view.peek_test(is_symbol_char) {
            view.advance();
            if view.peek_test(is_dot) && view.peek_next_test(is_symbol_start) {
                view.advance();
            }
        }
    }
    inner(view);
    if view.peek_test(is_slash) && view.peek_next_test(is_symbol_start) {
        view.advance();
        inner(view);
    }
    view.skim_token(TokenType::Symbol)
}

fn scan_keyword(view: &CharView) -> NResult<Token> {
    fn inner(view: &CharView) {
        while view.peek_test(is_symbol_char) {
            view.advance();
            if view.peek_test(is_dot) && view.peek_next_test(is_symbol_start) {
                view.advance();
            }
        }
    }
    inner(view);
    if view.peek_test(is_slash) && view.peek_next_test(is_symbol_start) {
        view.advance();
        inner(view);
    }
    view.skim_token(TokenType::Keyword)
}

fn scan_string(view: &CharView) -> NResult<Token> {
    while !view.peek_test(is_double_quote) && !view.eos() {
        view.advance();
    }
    view.advance();
    view.skim_token(TokenType::String)
}

fn scan_number(view: &CharView) -> NResult<Token> {
    fn inner(view: &CharView) {
        while view.peek_test(is_digit) {
            view.advance();
        }
    }
    inner(view);
    if view.peek_test(is_dot) && view.peek_next_test(is_digit) {
        view.advance();
        inner(view);
    }
    view.skim_token(TokenType::Number)
}

fn scan_indent(view: &CharView) -> NResult<Token> {
    while view.peek_test(is_whitespace) {
        view.advance();
    }
    view.skim_token(TokenType::Indent)
}

fn scan_comment(view: &CharView) -> NResult<Token> {
    // lets handle some comments.
    while !view.peek_test(is_newline) && !view.eos() {
        view.advance();
    }
    view.skim_issue(IssueType::Ok)
    // view.skim_token(TokenType::Comment)
}

pub fn scan(context: &Context, source: String) -> NResult<Vec<Token>> {
    let view = CharView::new(source.chars().collect());
    let mut tokens = vec![];
    let mut issues = vec![];
    while let Some(c) = view.advance() {
        use TokenType::*;
        match match c {
            '(' => view.skim_token(LeftParen),
            ')' => view.skim_token(RightParen),
            '{' => view.skim_token(LeftBrace),
            '}' => view.skim_token(RightBrace),
            '[' => view.skim_token(LeftBracket),
            ']' => view.skim_token(RightBracket),
            '\'' => view.skim_token(Quote),
            '/' => view.skim_token(Slash),
            '"' => scan_string(&view),
            ':' => {
                if view.peek_test(is_colon) && view.peek_next_test(is_symbol_start) {
                    view.advance();
                    scan_keyword(&view)
                } else if view.peek_test(is_symbol_start) {
                    scan_keyword(&view)
                } else {
                    view.skim_issue(IssueType::ScanError)
                }
            }
            '#' => {
                if view.advance_if_true(is_left_paren) {
                    view.skim_token(HashLeftParen)
                } else if view.advance_if_true(is_left_bracket) {
                    view.skim_token(HashLeftBracket)
                } else if view.advance_if_true(is_left_brace) {
                    view.skim_token(HashLeftBrace)
                } else {
                    view.skim_token(Hash)
                }
            }
            ' ' | '\t' | ',' | '\n' => {
                if *c == '\n' {
                    view.newline();
                }
                view.skim_issue(IssueType::Ok)
            }
            ';' => scan_comment(&view),
            _ if is_symbol_start(c) => scan_symbol(&view),
            _ if is_digit(c) => scan_number(&view),
            _ => view.skim_issue(IssueType::ScanError),
        } {
            Ok(token) => tokens.push(token),
            Err(issue) => {
                if issue.issue != IssueType::Ok {
                    issues.push(issue);
                    context.post_error("Failed to scan token");
                }
            }
        }
        view.reset();
    }
    tokens.push(view.skim_token(TokenType::Eof).unwrap());
    Ok(tokens)
}
