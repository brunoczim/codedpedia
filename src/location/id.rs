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

impl Clone for Box<Id> {
    fn clone(&self) -> Self {
        self.into_boxed()
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

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.raw_contents()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

#[cfg(test)]
mod test {
    use super::Id;

    #[test]
    fn valid_alphanumeric() {
        let id = Id::parse("hell0").unwrap();
        assert_eq!(id.raw_contents(), "hell0");
    }

    #[test]
    fn valid_slug() {
        let id = Id::parse("hello-world_yahoo").unwrap();
        assert_eq!(id.raw_contents(), "hello-world_yahoo");
    }

    #[test]
    fn invalid_space() {
        Id::parse("h a").unwrap_err();
    }

    #[test]
    fn invalid_bar() {
        Id::parse("ha/he").unwrap_err();
    }

    #[test]
    fn invalid_hash() {
        Id::parse("ha#he").unwrap_err();
    }
}
