#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Name,
    Number,
    Equal,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Plus,
    Minus,

    Whitespace,

    Unknown,
    EndOfFile,
}

#[derive(Clone, Copy)]
pub struct Token<'source> {
    pub token_type: TokenType,
    pub contents: &'source str,
}

#[derive(Clone)]
pub struct Lexer<'source> {
    source: &'source [u8],
}

impl<'source> Lexer<'source> {
    pub const fn new(source: &'source str) -> Self {
        Self {
            source: source.as_bytes(),
        }
    }

    fn read_bytes(&mut self, count: usize, token_type: TokenType) -> Token<'source> {
        let (contents, source) = self.source.split_at(count);

        self.source = source;

        Token {
            token_type,
            contents: std::str::from_utf8(contents).unwrap(),
        }
    }

    fn read_number(&mut self) -> Option<&'source str> {
        let mut iterator = self.source.iter().copied().enumerate();
        let (mut index, mut character) = iterator.next()?;

        if character.is_ascii_digit() {
            while character.is_ascii_digit() {
                if let Some((index_2, character_2)) = iterator.next() {
                    (index, character) = (index_2, character_2);
                } else {
                    index += 1;

                    break;
                }
            }

            if character == b'.' {
                (index, character) = iterator.next()?;

                while character.is_ascii_digit() {
                    if let Some((index_2, character_2)) = iterator.next() {
                        (index, character) = (index_2, character_2);
                    } else {
                        index += 1;

                        break;
                    }
                }
            }

            let (contents, source) = self.source.split_at(index);

            self.source = source;

            Some(std::str::from_utf8(contents).unwrap())
        } else {
            None
        }
    }

    fn read_name(&mut self) -> Option<&'source str> {
        let mut iterator = self.source.iter().copied().enumerate();
        let (mut index, mut character) = iterator.next()?;

        if character.is_ascii_alphabetic() || character == b'_' {
            while character.is_ascii_alphanumeric() || character == b'_' {
                if let Some((index_2, character_2)) = iterator.next() {
                    (index, character) = (index_2, character_2);
                } else {
                    index += 1;

                    break;
                }
            }

            let (contents, source) = self.source.split_at(index);

            self.source = source;

            Some(std::str::from_utf8(contents).unwrap())
        } else {
            None
        }
    }

    fn read_others(&mut self) -> Token<'source> {
        self.read_number().map_or_else(
            || {
                self.read_name().map_or_else(
                    || self.read_bytes(1, TokenType::Unknown),
                    |contents| Token {
                        token_type: TokenType::Name,
                        contents,
                    },
                )
            },
            |contents| Token {
                token_type: TokenType::Number,
                contents,
            },
        )
    }

    pub fn next(&mut self) -> Token<'source> {
        match self.source {
            [b'=', ..] => self.read_bytes(1, TokenType::Equal),
            [b'(', ..] => self.read_bytes(1, TokenType::LeftParenthesis),
            [b')', ..] => self.read_bytes(1, TokenType::RightParenthesis),
            [b',', ..] => self.read_bytes(1, TokenType::Comma),
            [b'+', ..] => self.read_bytes(1, TokenType::Plus),
            [b'-', ..] => self.read_bytes(1, TokenType::Minus),
            [b'\n' | b'\r' | b'\t' | b' ', ..] => self.read_bytes(1, TokenType::Whitespace),
            [] => Token {
                token_type: TokenType::EndOfFile,
                contents: "",
            },

            _ => self.read_others(),
        }
    }

    pub fn peek(&self, index: u8) -> Token<'source> {
        let mut lexer = self.clone();

        for _ in 0..index {
            lexer.next();
        }

        lexer.next()
    }

    fn next_if_just(&mut self, token_type: TokenType) -> Option<&'source str> {
        let token = self.peek(0);

        if token.token_type == token_type {
            self.next();

            Some(token.contents)
        } else {
            None
        }
    }

    pub fn next_if(&mut self, token_type: TokenType) -> Option<&'source str> {
        self.skip_whitespace();

        self.next_if_just(token_type)
    }

    pub fn skip_whitespace(&mut self) {
        while self.next_if_just(TokenType::Whitespace).is_some() {}
    }
}
