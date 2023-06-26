use crate::lexer::Lexer;
use crate::token::Token;
use serde::{Deserialize, Serialize};
use strum::Display;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Display)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Symbol {
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    LParen,   // (
    RParen,   // )
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum TokenJSObject {
    Number(f64), // 数字
    Operator(Symbol),
}

#[wasm_bindgen]
pub struct NodeLexer {
    lexer: Lexer,
}

#[wasm_bindgen]
impl NodeLexer {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        NodeLexer {
            lexer: Lexer::new(input),
        }
    }

    pub fn token(&mut self) -> Option<TokenJSObject> {
        let token = self.lexer.token();
        match token {
            Some(Token::Number(val)) => Some(TokenJSObject::Number(val)),
            Some(Token::Plus) => Some(TokenJSObject::Operator(Symbol::Plus)),
            Some(Token::Minus) => Some(TokenJSObject::Operator(Symbol::Minus)),
            Some(Token::Asterisk) => Some(TokenJSObject::Operator(Symbol::Asterisk)),
            Some(Token::Slash) => Some(TokenJSObject::Operator(Symbol::Slash)),
            Some(Token::LParen) => Some(TokenJSObject::Operator(Symbol::LParen)),
            Some(Token::RParen) => Some(TokenJSObject::Operator(Symbol::RParen)),
            None => None,
        }
    }
}

#[wasm_bindgen]
pub fn convert_symbol_to_string(symbol: Symbol) -> String {
    match symbol {
        Symbol::Plus => "+".to_string(),
        Symbol::Minus => "-".to_string(),
        Symbol::Asterisk => "*".to_string(),
        Symbol::Slash => "/".to_string(),
        Symbol::LParen => "(".to_string(),
        Symbol::RParen => ")".to_string(),
    }
}
