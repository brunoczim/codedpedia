use std::{error::Error, fmt, mem, rc::Rc, sync::Arc};

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
        Self::check_parse_input(input)?;
        Ok(Self::from_ref(input))
    }

    pub fn parse_owned(input: Box<str>) -> Result<Box<Self>, InvalidId> {
        Self::check_parse_input(input.as_ref())?;
        Ok(Self::from_box(input))
    }

    fn from_ref(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    fn from_box(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }

    fn check_parse_input(input: &str) -> Result<(), InvalidId> {
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

        Ok(())
    }
}

pub trait AsId {
    fn as_id(&self) -> &Id;
}

impl AsId for Id {
    fn as_id(&self) -> &Id {
        self
    }
}

impl<'this, I> AsId for &'this I
where
    I: AsId + ?Sized,
{
    fn as_id(&self) -> &Id {
        (**self).as_id()
    }
}

impl<I> AsId for Box<I>
where
    I: AsId + ?Sized,
{
    fn as_id(&self) -> &Id {
        (**self).as_id()
    }
}

impl<I> AsId for Rc<I>
where
    I: AsId + ?Sized,
{
    fn as_id(&self) -> &Id {
        (**self).as_id()
    }
}

impl<I> AsId for Arc<I>
where
    I: AsId + ?Sized,
{
    fn as_id(&self) -> &Id {
        (**self).as_id()
    }
}
