#[derive(Debug, Clone)]
pub struct StringNode {
    literal: String,
}

impl StringNode {
    pub fn from(lexeme: &str) -> StringNode {
        // we need to do some string escaping here
        let mut literal = String::new();
        let mut escape = false;
        for (i, c) in lexeme.chars().enumerate() {
            if i == 0 || i == lexeme.len() - 1 {
                continue;
            }
            if escape {
                escape = false;
                match c {
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    '0' => literal.push('\0'),
                    '\\' => literal.push('\\'),
                    c => literal.push(c),
                }
                continue;
            }
            if c == '\\' {
                escape = true;
                continue;
            }
            literal.push(c);
        }
        StringNode { literal }
    }

    pub fn value(&self) -> &str {
        &self.literal[..]
    }
}
