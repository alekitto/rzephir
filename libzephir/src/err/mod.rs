use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ActionsCannotBeEmptyError {}

impl ActionsCannotBeEmptyError {
    pub fn new() -> Self {
        ActionsCannotBeEmptyError {}
    }
}

impl fmt::Display for ActionsCannotBeEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actions vector cannot be empty")
    }
}

impl Error for ActionsCannotBeEmptyError {}

#[derive(Debug)]
pub struct UnknownPolicyVersionError {
    version: i32,
}

impl UnknownPolicyVersionError {
    pub fn new(version: i32) -> Self {
        UnknownPolicyVersionError { version }
    }
}

impl fmt::Display for UnknownPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown policy version")
    }
}

impl Error for UnknownPolicyVersionError {}
