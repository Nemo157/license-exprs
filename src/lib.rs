#[macro_use]
extern crate nom;

use std::error::Error;
use std::fmt;

use nom::IResult;

mod spdx;
mod expr;
mod parser;

use self::LicenseExpr::*;

#[derive(Debug, Clone, Copy)]
pub enum LicenseExpr<'a> {
    License(&'a str),
    Exception(&'a str),
    And, Or, With,
}

impl<'a> fmt::Display for LicenseExpr<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            With => format.write_str("WITH"),
            And  => format.write_str("AND"),
            Or   => format.write_str("OR"),
            License(info) | Exception(info) => format.write_str(info),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    InvalidStructure(LicenseExpr<'a>)
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::UnknownLicenseId(info)
                => format.write_fmt(format_args!("{}: {}", self.description(), info)),
            ParseError::InvalidStructure(info)
                => format.write_fmt(format_args!("{}: {}", self.description(), info)),
        }
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnknownLicenseId(_) => "unknown license or other term",
            ParseError::InvalidStructure(_) => "invalid license expression",
        }
    }
}

pub fn validate_license_expr(license_expr: &str) -> Result<(), ParseError> {
    let parsed = parser::compound(license_expr);
    println!("{:#?}", parsed);
    match parsed {
        IResult::Done(ref rest, _) if !rest.is_empty() => Err(ParseError::InvalidStructure(LicenseExpr::And)),
        IResult::Done(_, ref expr) if expr.is_valid() => Ok(()),
        IResult::Done(_, _) => Err(ParseError::UnknownLicenseId("")),
        IResult::Error(_) => Err(ParseError::InvalidStructure(LicenseExpr::And)),
        IResult::Incomplete(_) => Err(ParseError::InvalidStructure(LicenseExpr::And)),
    }
}

pub fn license_version() -> &'static str {
    spdx::VERSION
}
