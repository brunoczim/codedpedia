use super::{
    buf::{DefaultOwnedStr, TextBuf},
    parse::Parse,
};
use std::{error::Error, fmt};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<S = DefaultOwnedStr> {
    contents: S,
}

pub type IdRef<'id> = Id<&'id str>;

impl<S> Id<S>
where
    S: TextBuf,
{
    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn borrow(&self) -> IdRef {
        Id { contents: self.contents.as_str() }
    }

    pub fn into_default_owned(self) -> Id {
        Id { contents: self.contents.into_owned_str() }
    }
}

impl<S> AsRef<str> for Id<S>
where
    S: TextBuf,
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<S> PartialEq<str> for Id<S>
where
    S: TextBuf,
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'input> Parse<'input> for IdRef<'input> {
    type Error = InvalidId;

    fn parse(input: &'input str) -> Result<Self, Self::Error> {
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

        Ok(Self { contents: input })
    }
}
