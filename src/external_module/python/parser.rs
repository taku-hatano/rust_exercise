use crate::lexer::Lexer;
use crate::parser::{Expression, Parser};
use crate::token::Token;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub enum Operator {
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
}

#[pyclass]
pub struct NumberExpression {
    #[pyo3(get)]
    val: f64,
}

#[pymethods]
impl NumberExpression {
    #[new]
    fn new(val: f64) -> Self {
        NumberExpression { val }
    }
}

#[pyclass]
pub struct PrefixExpression {
    #[pyo3(get)]
    operator: Operator,
    #[pyo3(get)]
    right: PyObject,
}

#[pymethods]
impl PrefixExpression {
    #[new]
    fn new(operator: Operator, right: PyObject) -> Self {
        PrefixExpression { operator, right }
    }
}

#[pyclass]
pub struct InfixExpression {
    #[pyo3(get)]
    left: PyObject,
    #[pyo3(get)]
    operator: Operator,
    #[pyo3(get)]
    right: PyObject,
}

#[pymethods]
impl InfixExpression {
    #[new]
    fn new(left: PyObject, operator: Operator, right: PyObject) -> Self {
        InfixExpression {
            left,
            operator,
            right,
        }
    }
}

fn operator_token_to_py(token: Token) -> Operator {
    match token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::Asterisk => Operator::Asterisk,
        Token::Slash => Operator::Slash,
        _ => panic!("invalid operator token"),
    }
}

impl IntoPy<PyObject> for Expression {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Expression::Number(val) => NumberExpression::new(val).into_py(py),
            Expression::PrefixExpression { operator, right } => {
                PrefixExpression::new(operator_token_to_py(operator), right.into_py(py)).into_py(py)
            }
            Expression::InfixExpression {
                left,
                operator,
                right,
            } => InfixExpression::new(
                left.into_py(py),
                operator_token_to_py(operator),
                right.into_py(py),
            )
            .into_py(py),
        }
    }
}

#[pyclass]
pub struct PyParser {
    parser: Parser,
}

#[pymethods]
impl PyParser {
    #[new]
    fn new(input: &str) -> Self {
        PyParser {
            parser: Parser::new(Lexer::new(input)),
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        let ret = self.parser.parse();
        ret.map(|expr| *expr)
    }
}
