use std::{error::Error, fmt};

use super::{
    external::{AsExternal, InvalidExternal},
    id::{AsId, InvalidId},
    internal::{Internal, InvalidInternal},
    path::{AsPath, InvalidPath},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidLocation {
    InvalidInternal(InvalidInternal),
    InvalidExternal(InvalidExternal),
}

impl From<InvalidExternal> for InvalidLocation {
    fn from(error: InvalidExternal) -> Self {
        Self::InvalidExternal(error)
    }
}

impl From<InvalidInternal> for InvalidLocation {
    fn from(error: InvalidInternal) -> Self {
        Self::InvalidInternal(error)
    }
}

impl fmt::Display for InvalidLocation {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInternal(error) => fmt::Display::fmt(error, fmtr),
            Self::InvalidExternal(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidLocation {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location<I, P, E>
where
    I: AsId,
    P: AsPath,
    E: AsExternal,
{
    Internal(Internal<I, P>),
    External(E),
}

impl<I, P, E> Location<I, P, E>
where
    I: AsId,
    P: AsPath,
    E: AsExternal,
{
    pub fn parse<'input>(input: &'input str) -> Result<Self, InvalidLocation>
    where
        I: TryFrom<&'input str, Error = InvalidId>,
        P: TryFrom<&'input str, Error = InvalidPath>,
        E: TryFrom<&'input str, Error = InvalidExternal>,
    {
        if input.contains("://") {
            Ok(Self::External(E::try_from(input)?))
        } else {
            Ok(Self::Internal(Internal::try_from(input)?))
        }
    }
}

impl<'input, I, P, E> TryFrom<&'input str> for Location<I, P, E>
where
    I: AsId + TryFrom<&'input str, Error = InvalidId>,
    P: AsPath + TryFrom<&'input str, Error = InvalidPath>,
    E: AsExternal + TryFrom<&'input str, Error = InvalidExternal>,
{
    type Error = InvalidLocation;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}
