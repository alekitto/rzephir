use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    ActionsCannotBeEmptyError = 1,
    UnknownPolicyVersionError = 2,

    UnknownError = -1,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<dyn std::error::Error + Send + Sync>,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Error {
            kind,
            inner: error.into(),
        }
    }

    pub fn get_ref(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        Some(self.inner.as_ref())
    }

    pub fn get_mut(&mut self) -> Option<&mut (dyn std::error::Error + Send + Sync + 'static)> {
        Some(self.inner.as_mut())
    }

    pub fn into_inner(self) -> Option<Box<dyn std::error::Error + Send + Sync>> {
        Some(self.inner)
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn actions_cannot_be_empty() -> Self {
        Self::new(
            ErrorKind::ActionsCannotBeEmptyError,
            "Actions set cannot be empty",
        )
    }

    pub fn unknown_policy_version(version: i32) -> Self {
        Self::new(
            ErrorKind::UnknownError,
            UnknownPolicyVersionError { version },
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}

impl<T> From<T> for Error
where
    T: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(err: T) -> Self {
        crate::err::Error::new(ErrorKind::UnknownError, err)
    }
}

#[derive(Debug)]
struct UnknownPolicyVersionError {
    version: i32,
}

impl fmt::Display for UnknownPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown policy version")
    }
}

impl std::error::Error for UnknownPolicyVersionError {}
