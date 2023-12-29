use super::{
    id::{Id, InvalidId},
    path::{InvalidPath, Path},
};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidInternal {
    InvalidPath(InvalidPath),
    InvalidId(InvalidId),
}

impl fmt::Display for InvalidInternal {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPath(error) => fmt::Display::fmt(error, fmtr),
            Self::InvalidId(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl From<InvalidPath> for InvalidInternal {
    fn from(error: InvalidPath) -> Self {
        Self::InvalidPath(error)
    }
}

impl From<InvalidId> for InvalidInternal {
    fn from(error: InvalidId) -> Self {
        Self::InvalidId(error)
    }
}

impl Error for InvalidInternal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Internal<I, P>
where
    I: AsRef<Id>,
    P: AsRef<Path>,
{
    Id(I),
    Path(P),
    PathWithId(P, I),
}

impl<I, P> Internal<I, P>
where
    I: AsRef<Id>,
    P: AsRef<Path>,
{
    pub fn parse<'input>(input: &'input str) -> Result<Self, InvalidInternal>
    where
        I: TryFrom<&'input str, Error = InvalidId>,
        P: TryFrom<&'input str, Error = InvalidPath>,
    {
        match input.char_indices().find(|(_, ch)| *ch == '#').map(|(i, _)| i) {
            Some(i) => {
                let (path_str, id_str) = input.split_at(i);
                let id = I::try_from(id_str)?;
                if path_str.is_empty() {
                    Ok(Self::Id(id))
                } else {
                    let path = P::try_from(path_str)?;
                    Ok(Self::PathWithId(path, id))
                }
            },

            None => {
                let path = P::try_from("")?;
                Ok(Self::Path(path))
            },
        }
    }
}

impl<'input, I, P> TryFrom<&'input str> for Internal<I, P>
where
    I: AsRef<Id> + TryFrom<&'input str, Error = InvalidId>,
    P: AsRef<Path> + TryFrom<&'input str, Error = InvalidPath>,
{
    type Error = InvalidInternal;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}
