pub mod lexer {
    use std::error::Error;
    use std::fmt::{self, Display, Formatter};
    use std::iter::Peekable;
    use std::str::Chars;
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
    pub struct SymbolError;

    #[derive(Debug)]
    pub enum SpecialSymbol {
        // `<` and `>`
        LeftAngleBracket,
        RightAngleBracket,
        // Parenthesis `()`
        LeftParenthesis,
        RightParenthesis,
        // Curly Parenthesis `{}`
        LeftBrace,
        RightBrace,
        // Square Parenthesis `[]`
        LeftSquareParenthesis,
        RightSquareParenthesis,
        // `#`
        Sharp,
        // `=`
        Assign,

        // `==`
        Equal,
        //// `>`
        //Greater,
        //// `<`
        //Smaller,
        // `>=`
        GreaterOrEqual,
        // `<=`
        SmallerOrEqual,

        // ,
        Comma,
        // ;
        Semicolon,
    }

    impl FromStr for SpecialSymbol {
        type Err = SymbolError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "#" => Ok(SpecialSymbol::Sharp),
                "<" => Ok(SpecialSymbol::LeftAngleBracket),
                ">" => Ok(SpecialSymbol::RightAngleBracket),
                "(" => Ok(SpecialSymbol::LeftParenthesis),
                ")" => Ok(SpecialSymbol::RightParenthesis),
                "{" => Ok(SpecialSymbol::LeftBrace),
                "}" => Ok(SpecialSymbol::RightBrace),
                "[" => Ok(SpecialSymbol::LeftSquareParenthesis),
                "]" => Ok(SpecialSymbol::RightSquareParenthesis),
                "=" => Ok(SpecialSymbol::Assign),
                "==" => Ok(SpecialSymbol::Equal),
                ">=" => Ok(SpecialSymbol::GreaterOrEqual),
                "<=" => Ok(SpecialSymbol::SmallerOrEqual),
                "," => Ok(SpecialSymbol::Comma),
                ";" => Ok(SpecialSymbol::Semicolon),

                _ => Err(SymbolError),
            }
        }
    }

    impl Display for SymbolError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "invalid token for Symbol")
        }
    }

    // This is important for other errors to wrap this one.
    impl Error for SymbolError {
        fn description(&self) -> &str {
            "Error in symbol, invalid token"
        }

        fn cause(&self) -> Option<&dyn Error> {
            // Generic error, underlying cause isn't tracked.
            None
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct KeywordError;

    impl Display for KeywordError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "invalid token")
        }
    }

    // This is important for other errors to wrap this one.
    impl Error for KeywordError {
        fn description(&self) -> &str {
            "invalid token"
        }

        fn cause(&self) -> Option<&dyn Error> {
            // Generic error, underlying cause isn't tracked.
            None
        }
    }

    #[derive(Debug)]
    pub enum Keyword {
        // The `const` keyword
        Const,
        // The `enum` keyword
        Enum,
        // The `return` keyword
        Return,
        // The `new` keyword
        New,
        // The `delete` keyword
        Delete,
        // The `include` keyword
        Include,

        // Basic types
        // The `void` keyword
        Void,
        // The `int` keyword
        Int,
        // The `double` keyword
        Double,

        // The `do` keyword
        Do,
        // The `for` keyword
        For,
        // The `while` keyword
        While,
        // The `break` keyword
        Break,
        // The `continue` keyword
        Continue,
        // The `if` keyword
        If,
        // The `else` keyword
        Else,
        // The `switch` keyword
        Switch,
        // The `case` keyword
        Case,
    }

    impl FromStr for Keyword {
        type Err = KeywordError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "const" => Ok(Keyword::Const),
                "enum" => Ok(Keyword::Enum),
                "return" => Ok(Keyword::Return),
                "new" => Ok(Keyword::New),
                "delete" => Ok(Keyword::Delete),
                "include" => Ok(Keyword::Include),

                "void" => Ok(Keyword::Void),
                "int" => Ok(Keyword::Int),
                "double" => Ok(Keyword::Double),

                "do" => Ok(Keyword::Do),
                "for" => Ok(Keyword::For),
                "while" => Ok(Keyword::While),
                "break" => Ok(Keyword::Break),
                "continue" => Ok(Keyword::Continue),
                "if" => Ok(Keyword::If),
                "else" => Ok(Keyword::Else),
                "switch" => Ok(Keyword::Switch),
                "case" => Ok(Keyword::Case),

                _ => Err(KeywordError),
            }
        }
    }

    #[derive(Debug)]
    pub enum Token {
        Keyword(Keyword),
        // include +-*/&^=
        SpecialSymbol(SpecialSymbol),
        // include ,;
        Comment(String),
        NumberLiteral(String),
        StringLiteral(String),
        Identifier(String),
    }

    impl Display for Token {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Debug, Clone)]
    pub struct LexerError {
        details: String,
    }

    impl From<SymbolError> for LexerError {
        fn from(symbolError: SymbolError) -> Self {
            LexerError::new("SymbolError convert to LexerError")
        }
    }

    impl From<KeywordError> for LexerError {
        fn from(keywordError: KeywordError) -> Self {
            LexerError::new("KeywordError convert to LexerError")
        }
    }

    impl LexerError {
        fn new(msg: &str) -> Self {
            Self {
                details: msg.to_string(),
            }
        }
    }

    impl fmt::Display for LexerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for LexerError {
        fn description(&self) -> &str {
            &self.details
        }

        fn cause(&self) -> Option<&dyn Error> {
            // Generic error, underlying cause isn't tracked.
            None
        }
    }

    pub struct Lexer<'a> {
        pub tokens: Vec<Token>,
        buffer: Peekable<Chars<'a>>,
        // true for finish normally
        status: bool,
    }

    impl<'a> IntoIterator for Lexer<'a> {
        type Item = Token;
        type IntoIter = ::std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            self.tokens.into_iter()
        }
    }

    // impl<'a> means that it a template
    impl<'a> Lexer<'a> {
        // Initialize a Lexer
        pub fn new(buffer: &'a str) -> Lexer<'a> {
            Lexer {
                tokens: Vec::new(),
                buffer: buffer.chars().peekable(),
                status: false,
            }
        }

        fn next(&mut self) -> Result<char, LexerError> {
            match self.buffer.next() {
                Some(ch) => Ok(ch),
                None => {
                    self.status = true;
                    Err(LexerError::new("Finish"))
                }
            }
        }

        fn peek(&mut self) -> Option<char> {
            self.buffer.peek().copied()
        }

        fn push_token(&mut self, token: Token) {
            //println!("Token {:?}", token);
            self.tokens.push(token);
        }

        fn get_line(&mut self) -> Result<String, LexerError> {
            let mut buf = String::new();
            loop {
                let ch = self.next()?;
                match ch {
                    _ if ch.is_ascii_control() => {
                        break;
                    }
                    _ => {
                        buf.push(ch);
                    }
                }
            }

            Ok(buf)
        }

        pub fn lex(&mut self) -> Result<(), LexerError> {
            loop {
                // once a token
                let ch = self.next()?;
                //dbg!(&ch);
                match ch {
                    // literal string
                    '"' | '\'' => {
                        let mut s = String::new();
                        loop {
                            match self.next() {
                                Ok(n) => {
                                    if ch == n {
                                        // end of string
                                        break;
                                    } else {
                                        s.push(n);
                                    }
                                }
                                Err(_) => {
                                    return Err(LexerError::new("Unexpected string ends with EOF"))
                                }
                            };
                        }
                        // not yet related to position
                        self.tokens.push(Token::StringLiteral(s));
                    }
                    // comment or divide
                    '/' => {
                        // it could be
                        // 1. /* */ as block comment
                        // 2. / alone as special symbol
                        // 3. /= as special symbol
                        // 4. // as line comment
                        //let mut s = ch.to_string();
                        if let Some(n) = self.peek() {
                            match n {
                                '*' => {
                                    // it is a block comment
                                    let mut s = String::new();
                                    let mut state = 0;
                                    'outer: loop {
                                        if state == 2 {
                                            break 'outer;
                                        }
                                        let ch = self.next()?;
                                        match ch {
                                            '*' => {
                                                state = 1;
                                            }
                                            '/' => {
                                                if state == 1 {
                                                    state = 2;
                                                }
                                            }
                                            _ => {
                                                if state == 1 {
                                                    s.push('*');
                                                }
                                                s.push(ch);
                                                state = 0;
                                            }
                                        }
                                    }
                                    self.push_token(Token::Comment(s));
                                }
                                '=' => {
                                    let mut s = ch.to_string();
                                    s.push(n);
                                    self.push_token(Token::SpecialSymbol(SpecialSymbol::from_str(
                                        &s,
                                    )?));
                                }
                                '/' => {
                                    let s = self.get_line()?;
                                    self.push_token(Token::Comment(s));
                                }
                                _ => {
                                    // a single /
                                    self.push_token(Token::SpecialSymbol(FromStr::from_str(
                                        &ch.to_string(),
                                    )?));
                                }
                            }
                        }
                    }
                    // special symbol
                    '+' | '-' | '*' | '&' | '|' | '^' | '=' => {
                        // spcial symbol
                        // 1. alone as special symbol
                        // 2. combine with =
                        let mut s = ch.to_string();
                        if let Some('=') = self.peek() {
                            s.push('=');
                        }
                        self.push_token(Token::SpecialSymbol(FromStr::from_str(&s)?));
                    }
                    // decimal
                    _ if ch.is_digit(10) => {
                        let mut num = ch.to_string();
                        while self.peek().unwrap().is_digit(10) {
                            num.push(self.next()?);
                        }

                        if self.peek() == Some('.') {
                            num.push('.');
                            while let Some(n) = self.peek() {
                                if n.is_numeric() {
                                    num.push(self.next()?);
                                } else {
                                    break;
                                }
                            }
                        }

                        self.push_token(Token::NumberLiteral(num));
                    }
                    // identifier or keyword
                    _ if ch.is_alphabetic() => {
                        // it is a
                        // 1. Identifier
                        // 2. Keyword
                        let mut s = ch.to_string();
                        while let Some(c) = self.peek() {
                            if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '.' {
                                let c = self.next()?;
                                s.push(c);
                            } else {
                                break;
                            }
                        }

                        if let Ok(keyword) = Keyword::from_str(&s) {
                            //println!("Keyword {:?}", keyword);
                            self.push_token(Token::Keyword(keyword));
                        } else {
                            //println!("identifier {:?}", s);
                            self.push_token(Token::Identifier(s));
                        }
                    }
                    ' ' | '\n' => (),
                    // other special symbol
                    _ => {
                        if let Ok(sym) = FromStr::from_str(&ch.to_string()) {
                            self.push_token(Token::SpecialSymbol(sym));
                        } else {
                            dbg!(&ch);
                            return Err(LexerError::new("Unexpected symbol in the sequence"));
                        }
                    }
                }
            }
        }

        pub fn get_status(&self) -> bool {
            return self.status;
        }
    }

}
