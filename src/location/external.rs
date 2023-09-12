use std::{rc::Rc, sync::Arc};
use url::Url;

pub type InvalidExternal = url::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct External {
    pub url: Url,
}

impl External {
    pub fn parse(input: &str) -> Result<Self, InvalidExternal> {
        Ok(Self { url: Url::parse(input)? })
    }
}

impl<'input> TryFrom<&'input str> for External {
    type Error = InvalidExternal;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl TryFrom<Box<str>> for External {
    type Error = InvalidExternal;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Self::parse(&input[..])
    }
}

pub trait AsExternal {
    fn as_external(&self) -> &External;
}

impl AsExternal for External {
    fn as_external(&self) -> &External {
        self
    }
}

impl<'this, E> AsExternal for &'this E
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Box<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Rc<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Arc<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}
