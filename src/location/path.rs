use super::component::{Component, InvalidComponent};
use std::{error::Error, fmt, mem, str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidPath {
    MissingInitialBar,
    InvalidComponent(InvalidComponent),
}

impl From<InvalidComponent> for InvalidPath {
    fn from(error: InvalidComponent) -> Self {
        Self::InvalidComponent(error)
    }
}

impl fmt::Display for InvalidPath {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingInitialBar => {
                write!(fmtr, "missing initial bar '/' in path")
            },
            Self::InvalidComponent(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidPath {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingInitialBar => None,
            Self::InvalidComponent(error) => Some(error),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Path {
    contents: str,
}

impl Path {
    pub const ROOT: &Self = Self::from_ref_unchecked("/");

    pub fn parse(input: &str) -> Result<&Self, InvalidPath> {
        let stripped =
            input.strip_prefix("/").ok_or(InvalidPath::MissingInitialBar)?;

        for component_str in stripped.split('/') {
            Component::parse(component_str)?;
        }

        Ok(Self::from_ref_unchecked(input))
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidPath> {
        Self::parse(input.as_ref())?;
        Ok(Self::from_box_uncheckedd(input))
    }

    pub fn components(&self) -> Components {
        let mut inner = self.contents.split('/');
        inner.next();
        Components { inner }
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_uncheckedd(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl<'input> TryFrom<&'input str> for &'input Path {
    type Error = InvalidPath;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Path::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Path> {
    type Error = InvalidPath;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Path::parse_boxed(input)
    }
}

impl<'path> IntoIterator for &'path Path {
    type Item = &'path Component;
    type IntoIter = Components<'path>;

    fn into_iter(self) -> Self::IntoIter {
        self.components()
    }
}

#[derive(Debug, Clone)]
pub struct Components<'path> {
    inner: str::Split<'path, char>,
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path Component;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Component::from_ref_unchecked)
    }
}

impl<'path> DoubleEndedIterator for Components<'path> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(Component::from_ref_unchecked)
    }
}
