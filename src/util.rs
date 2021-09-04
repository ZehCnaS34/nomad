pub fn is_symbol_start(c: &char) -> bool {
    c.is_alphabetic() || "!$%^&*_-+<>':|".contains(*c)
}

pub fn is_symbol_char(c: &char) -> bool {
    c.is_alphabetic() || c.is_digit(10) || "!$%^&*_-+<>':|".contains(*c)
}

pub fn is_colon(c: &char) -> bool {
    *c == ':'
}

pub fn is_dot(c: &char) -> bool {
    *c == '.'
}

pub fn is_slash(c: &char) -> bool {
    *c == '/'
}

pub fn is_newline(c: &char) -> bool {
    *c == '\n'
}

pub fn is_left_paren(c: &char) -> bool {
    *c == '('
}
pub fn is_left_bracket(c: &char) -> bool {
    *c == '['
}
pub fn is_left_brace(c: &char) -> bool {
    *c == '{'
}

pub fn is_double_quote(c: &char) -> bool {
    *c == '"'
}

pub fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

pub fn is_whitespace(c: &char) -> bool {
    ' ' == *c || '\t' == *c || '\r' == *c
}
