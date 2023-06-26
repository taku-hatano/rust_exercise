use std::borrow::Borrow;
use std::io::{self, Write};

use rust_exercise::lexer::Lexer;
use rust_exercise::parser::{Expression, Parser};
use rust_exercise::token::Token;

fn evaluate(expr: &Expression) -> f64 {
    match expr {
        Expression::Number(val) => *val,
        Expression::PrefixExpression { operator, right } => {
            let right = evaluate(right);
            match operator {
                Token::Plus => right,
                Token::Minus => -right,
                _ => panic!("unknown operator: {:?}", operator),
            }
        }
        Expression::InfixExpression {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left);
            let right = evaluate(right);
            match operator {
                Token::Plus => left + right,
                Token::Minus => left - right,
                Token::Asterisk => left * right,
                Token::Slash => left / right,
                _ => panic!("unknown operator: {:?}", operator),
            }
        }
    }
}

fn main() {
    loop {
        print!("[rust]>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "exit\n" {
            break;
        }

        println!("\n**** Lexer Result ****");
        let mut lexer = Lexer::new(input.as_str());
        let mut token = lexer.token();
        while token.is_some() {
            println!("{:?}", token);
            token = lexer.token();
        }

        let lexer = Lexer::new(input.as_str());
        let mut parser = Parser::new(lexer);

        let expr = parser.parse();

        if let Some(expr) = expr {
            println!("\n**** Evaluate Result ****");
            println!("{}", evaluate(expr.borrow()));
        }
    }
}
