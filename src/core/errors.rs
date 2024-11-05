use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum Av1Errors {
    ConfigError,
}

impl fmt::Display for Av1Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for Av1Errors {}
