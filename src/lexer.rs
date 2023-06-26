use super::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    // 初期化
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn token(&mut self) -> Option<Token> {
        // 空白/改行をスキップする
        self.skip_whitespace();

        // 現在のトークンを取得する
        self.parse_token()
    }

    fn parse_token(&mut self) -> Option<Token> {
        let c = self.current()?;
        let token: Option<Token> = if self.is_number(c) {
            self.number()
        } else {
            self.operator()
        };
        self.next();
        token
    }

    fn number(&mut self) -> Option<Token> {
        // NOTE 先頭文字が数字でない場合は None を返す
        let c = self.current()?;
        if !self.is_number(c) {
            return None;
        }

        let mut number = vec![*c];
        while self.peek().is_some() && self.is_number(self.peek().unwrap()) {
            self.next();
            number.push(*self.current().unwrap());
        }
        String::from_iter(number)
            .parse::<f64>()
            .ok()
            .map(Token::Number)
    }

    fn operator(&mut self) -> Option<Token> {
        let c = self.current()?;
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            _ => None,
        }
    }

    // 空白/改行をスキップする
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    // 次のインデックスに進む
    fn next(&mut self) {
        self.position += 1;
    }

    // 現在の文字を取得する
    fn current(&self) -> Option<&char> {
        self.input.get(self.position)
    }

    // 次に解析する文字
    fn peek(&self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    // 数字かどうかの判定
    fn is_number(&self, c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

#[test]
fn test_number_lexer() {
    let mut lexer = Lexer::new("1 + 2.2 * (-3)");
    assert_eq!(lexer.token(), Some(Token::Number(1_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2.2_f64)));
    assert_eq!(lexer.token(), Some(Token::Asterisk));
    assert_eq!(lexer.token(), Some(Token::LParen));
    assert_eq!(lexer.token(), Some(Token::Minus));
    assert_eq!(lexer.token(), Some(Token::Number(3_f64)));
    assert_eq!(lexer.token(), Some(Token::RParen));
    assert_eq!(lexer.token(), None);
}
