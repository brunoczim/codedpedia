use std::{error::Error, fmt, mem};

use super::{
    external::{External, InvalidExternal},
    internal::{Internal, InvalidInternal},
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

pub fn parse<'a>(
    input: &'a str,
) -> Result<(&'a Location, ViewRef<'a>), InvalidLocation> {
    let view = if input.contains("://") {
        let external = External::parse(input)?;
        ViewRef::External(external)
    } else {
        let internal = Internal::parse(input)?;
        ViewRef::Internal(internal)
    };
    let location = Location::from_ref_unchecked(input);
    Ok((location, view))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    contents: str,
}

impl Location {
    pub fn parse(input: &str) -> Result<&Self, InvalidLocation> {
        let (external_loc, _) = parse(input)?;
        Ok(external_loc)
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidLocation> {
        Self::parse(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn into_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl Clone for Box<Location> {
    fn clone(&self) -> Self {
        self.into_boxed()
    }
}

impl<'a> From<&'a Location> for Box<Location> {
    fn from(reference: &'a Location) -> Self {
        reference.into_boxed()
    }
}

impl PartialEq<str> for Location {
    fn eq(&self, other: &str) -> bool {
        Self::parse(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input Location {
    type Error = InvalidLocation;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Location::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Location> {
    type Error = InvalidLocation;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Location::parse_boxed(input)
    }
}

impl AsRef<Location> for Location {
    fn as_ref(&self) -> &Location {
        self
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl ToOwned for Location {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        self.into_boxed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewRef<'a> {
    Internal(&'a Internal),
    External(&'a External),
}

impl<'a> TryFrom<&'a str> for ViewRef<'a> {
    type Error = InvalidLocation;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> ViewRef<'a> {
    pub fn parse(input: &'a str) -> Result<Self, InvalidLocation> {
        let (_, view) = parse(input)?;
        Ok(view)
    }

    pub fn to_boxed(&self) -> Box<Location> {
        let location_str = self.to_string();
        Location::from_box_unchecked(location_str.into_boxed_str())
    }
}

impl<'a> fmt::Display for ViewRef<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal(internal) => write!(fmtr, "{}", internal),
            Self::External(external) => write!(fmtr, "{}", external),
        }
    }
}
