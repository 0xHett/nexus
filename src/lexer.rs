pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn lex(file_string_in: &'a String) -> Self {
        Lexer {
            input: file_string_in.chars().peekable(),
            line: 1,
        }
    }

    pub fn read() {
        
    }
}
