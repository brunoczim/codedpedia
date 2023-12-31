use std::{error::Error, fmt};

use super::{
    external::{External, InvalidExternal},
    id::{Id, InvalidId},
    internal::{Internal, InvalidInternal, View},
    path::{InvalidPath, Path},
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
pub enum Location {
    Internal(Box<Internal>),
    External(External),
}

/*
impl<P, I, E> Location<P, I, E>
where
    P: AsRef<Path>,
    I: AsRef<Id>,
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
            Ok(Self::Internal(View::try_from(input)?))
        }
    }
}
*/

/*
impl<'input, P, I, E> TryFrom<&'input str> for Location<P, I, E>
where
    P: AsRef<Path> + TryFrom<&'input str, Error = InvalidPath>,
    I: AsRef<Id> + TryFrom<&'input str, Error = InvalidId>,
    E: AsExternal + TryFrom<&'input str, Error = InvalidExternal>,
{
    type Error = InvalidLocation;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}
*/
