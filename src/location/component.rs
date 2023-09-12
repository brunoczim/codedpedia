use std::{error::Error, fmt, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidComponent {
    Empty,
    CurrentDir,
    ParentDir,
    Bar,
    Hash,
}

impl fmt::Display for InvalidComponent {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmtr, "Path component cannot be empty"),
            Self::CurrentDir => {
                write!(
                    fmtr,
                    "Path compoonent cannot reference current directory"
                )
            },
            Self::ParentDir => {
                write!(fmtr, "Path component cannot reference parent directory")
            },
            Self::Bar => {
                write!(fmtr, "Path component cannot contain a bar '/'")
            },
            Self::Hash => {
                write!(fmtr, "Path component cannot contain a hash '#'")
            },
        }
    }
}

impl Error for InvalidComponent {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Component {
    contents: str,
}

impl Component {
    pub fn parse(input: &str) -> Result<&Self, InvalidComponent> {
        if input.is_empty() {
            Err(InvalidComponent::Empty)?;
        }
        if input == "." {
            Err(InvalidComponent::CurrentDir)?;
        }
        if input == ".." {
            Err(InvalidComponent::ParentDir)?;
        }

        for ch in input.chars() {
            if ch == '/' {
                Err(InvalidComponent::Bar)?;
            }
            if ch == '#' {
                Err(InvalidComponent::Hash)?;
            }
        }

        Ok(Self::from_ref_unchecked(input))
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidComponent> {
        Self::parse(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl<'input> TryFrom<&'input str> for &'input Component {
    type Error = InvalidComponent;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Component::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Component> {
    type Error = InvalidComponent;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Component::parse_boxed(input)
    }
}
