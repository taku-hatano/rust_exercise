#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64), // 数字
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    LParen,      // (
    RParen,      // )
}
