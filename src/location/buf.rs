use std::{rc::Rc, sync::Arc};

pub trait TextBuf: Clone {
    fn as_str(&self) -> &str;

    fn into_owned_str(self) -> DefaultOwnedStr;
}

impl<'s> TextBuf for &'s str {
    fn as_str(&self) -> &str {
        *self
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        DefaultOwnedStr { buf: self.into() }
    }
}

impl TextBuf for Box<str> {
    fn as_str(&self) -> &str {
        &self[..]
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        DefaultOwnedStr { buf: self }
    }
}

impl TextBuf for String {
    fn as_str(&self) -> &str {
        &self[..]
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        DefaultOwnedStr { buf: self.into() }
    }
}

impl TextBuf for Rc<str> {
    fn as_str(&self) -> &str {
        &self[..]
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        DefaultOwnedStr { buf: Box::from(self.as_str()) }
    }
}

impl TextBuf for Arc<str> {
    fn as_str(&self) -> &str {
        &self[..]
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        DefaultOwnedStr { buf: Box::from(self.as_str()) }
    }
}

impl TextBuf for DefaultOwnedStr {
    fn as_str(&self) -> &str {
        &self.buf[..]
    }

    fn into_owned_str(self) -> DefaultOwnedStr {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultOwnedStr {
    buf: Box<str>,
}

impl PartialEq<str> for DefaultOwnedStr {
    fn eq(&self, other: &str) -> bool {
        &self.buf[..] == other
    }
}

impl<'s> From<&'s str> for DefaultOwnedStr {
    fn from(string: &'s str) -> Self {
        string.into_owned_str()
    }
}

impl From<Box<str>> for DefaultOwnedStr {
    fn from(buf: Box<str>) -> Self {
        buf.into_owned_str()
    }
}

impl From<String> for DefaultOwnedStr {
    fn from(buf: String) -> Self {
        buf.into_owned_str()
    }
}

impl From<Rc<str>> for DefaultOwnedStr {
    fn from(buf: Rc<str>) -> Self {
        buf.into_owned_str()
    }
}

impl From<Arc<str>> for DefaultOwnedStr {
    fn from(buf: Arc<str>) -> Self {
        buf.into_owned_str()
    }
}

impl AsRef<Self> for DefaultOwnedStr {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for DefaultOwnedStr {
    fn as_ref(&self) -> &str {
        self.buf.as_ref()
    }
}
