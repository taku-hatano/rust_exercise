use super::lexer::Lexer;
use super::token::Token;
use std::borrow::Borrow;
use std::mem;

#[derive(Debug)]
pub enum Expression {
    Number(f64), // 数字
    PrefixExpression {
        operator: Token,        // 演算子
        right: Box<Expression>, // 右辺
    },
    InfixExpression {
        left: Box<Expression>,  // 左辺
        operator: Token,        // 演算子
        right: Box<Expression>, // 右辺
    },
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Sum,
    Product,
    Prefix,
}

pub struct Parser {
    lexer: Lexer,
    current: Option<Token>,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current = lexer.token();
        let peek = lexer.token();
        Parser {
            lexer,
            current,
            peek,
        }
    }

    pub fn parse(&mut self) -> Option<Box<Expression>> {
        self.parse_expression(Precedence::Lowest)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expression>> {
        let mut left = self.parse_prefix()?;

        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        Some(left)
    }

    fn parse_prefix(&mut self) -> Option<Box<Expression>> {
        match self.current.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }

    fn parse_minus(&mut self) -> Option<Box<Expression>> {
        self.next();
        let number = self.parse_expression(Precedence::Prefix)?;
        Some(Box::new(Expression::PrefixExpression {
            operator: Token::Minus,
            right: number,
        }))
    }

    fn parse_number(&mut self) -> Option<Box<Expression>> {
        match self.current.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expression::Number(*n))),
            _ => None,
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Expression>> {
        self.next();
        let expression = self.parse_expression(Precedence::Lowest);
        if self.is_peek(&Token::RParen) {
            self.next();
            expression
        } else {
            None
        }
    }

    fn parse_infix(&mut self, left: Box<Expression>) -> Option<Box<Expression>> {
        let token = self.current.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }

    fn parse_infix_expression(&mut self, left: Box<Expression>) -> Option<Box<Expression>> {
        let token = self.current.as_ref()?;
        let operator = token.clone();
        let precedence = self.token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        Some(Box::new(Expression::InfixExpression {
            left,
            operator,
            right,
        }))
    }

    fn next(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.token();
    }

    fn is_peek(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }

    // 次のトークンの優先度を取得
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::Lowest;
        }
        self.token_precedence(token.as_ref().unwrap())
    }

    // トークンの優先度を取得
    fn token_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

#[test]
fn test_parser() {
    do_parser(
        "1 + 2",
        r#"Some(InfixExpression { left: Number(1.0), operator: Plus, right: Number(2.0) })"#,
    );
    do_parser(
        "- 1 + 2 * 3",
        r#"Some(InfixExpression { left: PrefixExpression { operator: Minus, right: Number(1.0) }, operator: Plus, right: InfixExpression { left: Number(2.0), operator: Asterisk, right: Number(3.0) } })"#,
    );

    do_parser(
        "1 * (2 + 3)",
        r#"Some(InfixExpression { left: Number(1.0), operator: Asterisk, right: InfixExpression { left: Number(2.0), operator: Plus, right: Number(3.0) } })"#,
    );
}

#[cfg(test)]
fn do_parser(input: &str, expect: &str) {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    assert_eq!(format!("{:?}", parser.parse()), expect);
}
