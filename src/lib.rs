#![allow(clippy::unused_unit)]

//! SQL parser.
use crate::formatter::Format;
/// This module contains the function `format`.
use crate::query::parse_statements;

pub mod character;
pub mod clause;
pub mod comment;
pub mod data_type;
pub mod expression;
pub mod formatter;
pub mod function;
pub mod identifier;
pub mod insert;
pub mod keyword;
//pub mod lint;
pub mod list;
pub mod numeric;
pub mod query;
pub mod select;
pub mod sequence;
pub mod set;
pub mod statement;
pub mod table;
pub mod term;
pub mod r#type;
pub mod update;
pub mod ws;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParsingError(String),
    ParsingIncompleteError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::ParsingError(e) => write!(f, "Unable to parse SQL: {}", e),
            Self::ParsingIncompleteError(s) => write!(f, "Unable to parse SQL: '{}'", s),
        }
    }
}

type FResult<T> = std::result::Result<T, Error>;

pub fn format(s: &str) -> FResult<String> {
    match parse_statements(s) {
        Err(e) => Err(Error::ParsingError(e.to_string())),
        Ok((remaining, ast)) => {
            if remaining.is_empty() {
                Ok(ast.output())
            } else {
                Err(Error::ParsingIncompleteError(remaining.to_string()))
            }
        }
    }
}

#[wasm_bindgen(catch)]
pub fn format_sql(s: &str) -> Result<String, JsValue> {
    match format(s) {
        Ok(s) => Ok(s),
        Err(e) => Err(JsValue::from(e.to_string())),
    }
}

#[cfg(test)]
mod tests;
