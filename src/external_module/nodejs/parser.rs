use crate::lexer::Lexer;
use crate::parser::{Expression, Parser};
use crate::token::Token;
use serde::{Deserialize, Serialize};
use strum::Display;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Display)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Operator {
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ExpressionJSObject {
    Number(f64), // 数字
    PrefixExpression {
        operator: Operator,             // 演算子
        right: Box<ExpressionJSObject>, // 右辺
    },
    InfixExpression {
        left: Box<Self>,                // 左辺
        operator: Operator,             // 演算子
        right: Box<ExpressionJSObject>, // 右辺
    },
}

#[wasm_bindgen]
pub struct NodeParser {
    parser: Parser,
}

#[wasm_bindgen]
impl NodeParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        NodeParser {
            parser: Parser::new(Lexer::new(input)),
        }
    }

    pub fn parse(&mut self) -> Option<ExpressionJSObject> {
        let expression = self.parser.parse();
        expression.map(|expr| *self.parse_expression_to_js(*expr))
    }

    fn parse_expression_to_js(&mut self, expr: Expression) -> Box<ExpressionJSObject> {
        match expr {
            Expression::Number(val) => Box::new(ExpressionJSObject::Number(val)),
            Expression::PrefixExpression { operator, right } => {
                Box::new(ExpressionJSObject::PrefixExpression {
                    operator: self.parse_operator_token_to_js(operator),
                    right: self.parse_expression_to_js(*right),
                })
            }
            Expression::InfixExpression {
                left,
                operator,
                right,
            } => Box::new(ExpressionJSObject::InfixExpression {
                left: self.parse_expression_to_js(*left),
                operator: self.parse_operator_token_to_js(operator),
                right: self.parse_expression_to_js(*right),
            }),
        }
    }

    fn parse_operator_token_to_js(&mut self, token: Token) -> Operator {
        match token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            Token::Asterisk => Operator::Asterisk,
            Token::Slash => Operator::Slash,
            _ => panic!("invalid operator token"),
        }
    }
}
