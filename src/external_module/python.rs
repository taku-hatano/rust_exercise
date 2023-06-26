mod lexer;
mod parser;
use pyo3::prelude::*;

#[pymodule]
fn rust_exercise(_py: Python, m: &PyModule) -> PyResult<()> {
    // lexer
    m.add_class::<lexer::PyLexer>()?;
    m.add_class::<lexer::Symbol>()?;
    m.add_class::<lexer::NumberToken>()?;
    m.add_class::<lexer::OperatorToken>()?;

    // parser
    m.add_class::<parser::PyParser>()?;
    m.add_class::<parser::Operator>()?;
    m.add_class::<parser::NumberExpression>()?;
    m.add_class::<parser::PrefixExpression>()?;
    m.add_class::<parser::InfixExpression>()?;
    Ok(())
}
