use crate::lexer::Lexer;
use crate::token::Token;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub enum Symbol {
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    LParen,   // (
    RParen,   // )
}

#[pyclass]
pub struct NumberToken {
    #[pyo3(get)]
    val: f64,
}

#[pymethods]
impl NumberToken {
    #[new]
    fn new(val: f64) -> Self {
        NumberToken { val }
    }
}

#[pyclass]
pub struct OperatorToken {
    #[pyo3(get)]
    val: Symbol,
}

#[pymethods]
impl OperatorToken {
    #[new]
    fn new(val: Symbol) -> Self {
        OperatorToken { val }
    }
}

impl IntoPy<PyObject> for Token {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Number(val) => NumberToken::new(val).into_py(py),
            Self::Plus => OperatorToken::new(Symbol::Plus).into_py(py),
            Self::Minus => OperatorToken::new(Symbol::Minus).into_py(py),
            Self::Asterisk => OperatorToken::new(Symbol::Asterisk).into_py(py),
            Self::Slash => OperatorToken::new(Symbol::Slash).into_py(py),
            Self::LParen => OperatorToken::new(Symbol::LParen).into_py(py),
            Self::RParen => OperatorToken::new(Symbol::RParen).into_py(py),
        }
    }
}

#[pyclass]
pub struct PyLexer {
    lexer: Lexer,
}

#[pymethods]
impl PyLexer {
    #[new]
    fn new(input: &str) -> Self {
        PyLexer {
            lexer: Lexer::new(input),
        }
    }

    pub fn token(&mut self) -> Option<Token> {
        self.lexer.token()
    }
}
