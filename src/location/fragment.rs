use super::{
    buf::{DefaultOwnedStr, TextBuf},
    parse::Parse,
};
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidFragment {
    Empty,
    CurrentDir,
    ParentDir,
    Bar,
    Hash,
}

impl fmt::Display for InvalidFragment {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmtr, "Fragment cannot be empty"),
            Self::CurrentDir => {
                write!(fmtr, "Fraagment cannot reference current directory")
            },
            Self::ParentDir => {
                write!(fmtr, "Fragment cannot reference parent directory")
            },
            Self::Bar => write!(fmtr, "Fragment cannot contain a bar '/'"),
            Self::Hash => write!(fmtr, "Fragment cannot contain a hash '#'"),
        }
    }
}

impl Error for InvalidFragment {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment<S = DefaultOwnedStr> {
    contents: S,
}

pub type FragmentRef<'frag> = Fragment<&'frag str>;

impl<S> Fragment<S>
where
    S: TextBuf,
{
    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn borrow(&self) -> FragmentRef {
        Fragment { contents: self.contents.as_str() }
    }

    pub fn into_default_owned(self) -> Fragment {
        Fragment { contents: self.contents.into_owned_str() }
    }
}

impl<S> AsRef<str> for Fragment<S>
where
    S: TextBuf,
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<S> PartialEq<str> for Fragment<S>
where
    S: TextBuf,
{
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<'input> Parse<'input> for FragmentRef<'input> {
    type Error = InvalidFragment;

    fn parse(input: &'input str) -> Result<Self, Self::Error> {
        if input.is_empty() {
            Err(InvalidFragment::Empty)?;
        }
        if input == "." {
            Err(InvalidFragment::CurrentDir)?;
        }
        if input == ".." {
            Err(InvalidFragment::ParentDir)?;
        }

        for ch in input.chars() {
            if ch == '/' {
                Err(InvalidFragment::Bar)?;
            }
            if ch == '#' {
                Err(InvalidFragment::Hash)?;
            }
        }

        Ok(Self { contents: input })
    }
}
