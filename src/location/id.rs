use std::{error::Error, fmt, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidId {
    Empty,
    InvalidStart(char),
    InvalidChar(char),
}

impl fmt::Display for InvalidId {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmtr, "Id cannot be empty"),
            Self::InvalidStart(ch) => write!(
                fmtr,
                "Id must start with an ASCII letter, found {:?}",
                ch
            ),
            Self::InvalidChar(ch) => write!(
                fmtr,
                "Id must contain only ASCII letters, digits, '_' or '-', \
                 found {:?}",
                ch
            ),
        }
    }
}

impl Error for InvalidId {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id {
    contents: str,
}

impl Id {
    pub fn parse(input: &str) -> Result<&Self, InvalidId> {
        let mut iter = input.chars();

        let ch = iter.next().ok_or(InvalidId::Empty)?;
        if !ch.is_ascii_alphabetic() {
            Err(InvalidId::InvalidStart(ch))?;
        }

        for ch in iter {
            if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-' {
                Err(InvalidId::InvalidChar(ch))?;
            }
        }

        Ok(Self::from_ref_unchecked(input))
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidId> {
        Self::parse(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl<'input> TryFrom<&'input str> for &'input Id {
    type Error = InvalidId;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Id::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Id> {
    type Error = InvalidId;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Id::parse_boxed(input)
    }
}

impl AsRef<Self> for Id {
    fn as_ref(&self) -> &Self {
        self
    }
}
