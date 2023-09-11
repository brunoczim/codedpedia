use super::{
    buf::DefaultOwnedStr,
    fragment::{Fragment, FragmentRef},
    id::{Id, IdRef, InvalidId},
    parse::Parse,
    path::{InvalidPath, Path, PathFragmentsRef, PathRef},
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Internal<FS = Vec<Fragment>, I = DefaultOwnedStr> {
    Path(Path<FS>),
    Id(Id<I>),
    PathWithId(Path<FS>, Id<I>),
}

pub type InternalRef<'path, 'id, F = Fragment> =
    Internal<PathRef<'path, F>, IdRef<'id>>;

pub type InternalRefFragmentsRef<'path, 'frag, 'id> =
    InternalRef<'path, 'id, FragmentRef<'frag>>;

pub type InternalFragmentsRef<'frag, 'id> =
    Internal<PathFragmentsRef<'frag>, IdRef<'id>>;

impl<'input> Parse<'input> for InternalFragmentsRef<'input, 'input> {
    type Error = InvalidInternal;

    fn parse(input: &'input str) -> Result<Self, Self::Error> {
        match input.char_indices().find(|(_, ch)| *ch == '#').map(|(i, _)| i) {
            Some(i) => {
                let (path_str, id_str) = input.split_at(i);
                let id = Id::parse(id_str)?;
                if path_str.is_empty() {
                    Ok(Self::Id(id))
                } else {
                    let path = Id::parse(path_str)?;
                    Ok(Self::PathWithId(path, id))
                }
            },
            None => Ok(Self::Path(Path::ROOT)),
        }
    }
}
