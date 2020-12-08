use rustpython_ast::Location;

use alloc::string::String;
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug)]
pub struct CompileError {
    pub error: CompileErrorType,
    pub location: Location,
    pub source_path: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CompileErrorType {
    /// Invalid assignment, cannot store value in target.
    Assign(&'static str),
    /// Invalid delete
    Delete(&'static str),
    /// Expected an expression got a statement
    ExpectExpr,
    SyntaxError(String),
    /// Multiple `*` detected
    MultipleStarArgs,
    /// Misplaced `*` expression
    InvalidStarExpr,
    /// Break statement outside of loop.
    InvalidBreak,
    /// Continue statement outside of loop.
    InvalidContinue,
    InvalidReturn,
    InvalidYield,
    InvalidYieldFrom,
    InvalidAwait,
    AsyncYieldFrom,
    AsyncReturnValue,
    InvalidFuturePlacement,
    InvalidFutureFeature(String),
    FunctionImportStar,
}

impl fmt::Display for CompileErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileErrorType::Assign(target) => write!(f, "can't assign to {}", target),
            CompileErrorType::Delete(target) => write!(f, "can't delete {}", target),
            CompileErrorType::ExpectExpr => write!(f, "Expecting expression, got statement"),
            CompileErrorType::SyntaxError(err) => write!(f, "{}", err.as_str()),
            CompileErrorType::MultipleStarArgs => {
                write!(f, "two starred expressions in assignment")
            }
            CompileErrorType::InvalidStarExpr => write!(f, "can't use starred expression here"),
            CompileErrorType::InvalidBreak => write!(f, "'break' outside loop"),
            CompileErrorType::InvalidContinue => write!(f, "'continue' outside loop"),
            CompileErrorType::InvalidReturn => write!(f, "'return' outside function"),
            CompileErrorType::InvalidYield => write!(f, "'yield' outside function"),
            CompileErrorType::InvalidYieldFrom => write!(f, "'yield from' outside function"),
            CompileErrorType::InvalidAwait => write!(f, "'await' outside async function"),
            CompileErrorType::AsyncYieldFrom => write!(f, "'yield from' inside async function"),
            CompileErrorType::AsyncReturnValue => {
                write!(f, "'return' with value inside async generator")
            }
            CompileErrorType::InvalidFuturePlacement => write!(
                f,
                "from __future__ imports must occur at the beginning of the file"
            ),
            CompileErrorType::InvalidFutureFeature(feat) => {
                write!(f, "future feature {} is not defined", feat)
            }
            CompileErrorType::FunctionImportStar => {
                write!(f, "import * only allowed at module level")
            }
        }
    }
}

#[cfg(feature = "std")]
impl Error for CompileErrorType {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}", self.error, self.location)
    }
}

#[cfg(feature = "std")]
impl Error for CompileError {}
