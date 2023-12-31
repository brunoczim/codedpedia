use std::{error::Error, fmt, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidExternal {
    MissingScheme,
    EmptyScheme,
    InvalidSchemeStart(char),
    InvalidSchemeChar(char),
}

impl fmt::Display for InvalidExternal {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingScheme => {
                write!(fmtr, "missing external location scheme")
            },
            Self::EmptyScheme => {
                write!(fmtr, "external location scheme cannot be empty")
            },
            Self::InvalidSchemeStart(ch) => write!(
                fmtr,
                "invalid starting character {:?} in external location scheme",
                ch
            ),
            Self::InvalidSchemeChar(ch) => write!(
                fmtr,
                "invalid character {:?} in external location scheme",
                ch
            ),
        }
    }
}

impl Error for InvalidExternal {}

pub fn parse<'a>(
    input: &'a str,
) -> Result<(&'a External, ViewRef<'a>), InvalidExternal> {
    let (scheme, rest) =
        input.split_once("://").ok_or(InvalidExternal::MissingScheme)?;

    let mut iter = scheme.chars();

    let ch = iter.next().ok_or(InvalidExternal::EmptyScheme)?;
    if !ch.is_ascii_alphabetic() {
        Err(InvalidExternal::InvalidSchemeStart(ch))?;
    }

    for ch in iter {
        if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-' {
            Err(InvalidExternal::InvalidSchemeChar(ch))?;
        }
    }

    let external_loc = External::from_ref_unchecked(input);

    let view = if scheme == "other" {
        ViewRef::Other(rest)
    } else {
        ViewRef::WithHost { scheme, rest }
    };

    Ok((external_loc, view))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct External {
    contents: str,
}

impl External {
    pub fn new(input: &str) -> Result<&Self, InvalidExternal> {
        let (external_loc, _) = parse(input)?;
        Ok(external_loc)
    }

    pub fn new_boxed(input: Box<str>) -> Result<Box<Self>, InvalidExternal> {
        Self::new(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn to_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn view(&self) -> ViewRef {
        let Some((scheme, rest)) = self.raw_contents().split_once("://") else {
            unreachable!()
        };

        if scheme == "other" {
            ViewRef::Other(rest)
        } else {
            ViewRef::WithHost { scheme, rest }
        }
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }

    pub(crate) fn into_boxed_contents(self: Box<Self>) -> Box<str> {
        unsafe { mem::transmute(self) }
    }
}

impl Clone for Box<External> {
    fn clone(&self) -> Self {
        self.to_boxed()
    }
}

impl<'a> From<&'a External> for Box<External> {
    fn from(reference: &'a External) -> Self {
        reference.to_boxed()
    }
}

impl PartialEq<str> for External {
    fn eq(&self, other: &str) -> bool {
        Self::new(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input External {
    type Error = InvalidExternal;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        External::new(input)
    }
}

impl TryFrom<Box<str>> for Box<External> {
    type Error = InvalidExternal;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        External::new_boxed(input)
    }
}

impl AsRef<External> for External {
    fn as_ref(&self) -> &External {
        self
    }
}

impl fmt::Display for External {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl ToOwned for External {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        self.to_boxed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewRef<'a> {
    WithHost { scheme: &'a str, rest: &'a str },
    Other(&'a str),
}

impl<'a> TryFrom<&'a str> for ViewRef<'a> {
    type Error = InvalidExternal;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> ViewRef<'a> {
    pub fn parse(input: &'a str) -> Result<Self, InvalidExternal> {
        let (_, view) = parse(input)?;
        Ok(view)
    }

    pub fn to_boxed(&self) -> Box<External> {
        let external_str = self.to_string();
        External::from_box_unchecked(external_str.into_boxed_str())
    }
}

impl<'a> fmt::Display for ViewRef<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::WithHost { scheme, rest } => {
                write!(fmtr, "{}://{}", scheme, rest)
            },
            Self::Other(rest) => write!(fmtr, "other://{}", rest),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::location::external::ViewRef;

    #[test]
    fn valid_with_host() {
        let (external_loc, view) =
            super::parse("https://duckduckgo.com/").unwrap();
        assert_eq!(external_loc.raw_contents(), "https://duckduckgo.com/");
        assert_eq!(
            view,
            ViewRef::WithHost { scheme: "https", rest: "duckduckgo.com/" }
        );
    }

    #[test]
    fn valid_as_other() {
        let (external_loc, view) =
            super::parse("other://urn:isbn:123").unwrap();
        assert_eq!(external_loc.raw_contents(), "other://urn:isbn:123");
        assert_eq!(view, ViewRef::Other("urn:isbn:123"));
    }

    #[test]
    fn invalid_no_scheme() {
        super::parse("abcd").unwrap_err();
    }

    #[test]
    fn invalid_empty_scheme() {
        super::parse("://abcd").unwrap_err();
    }

    #[test]
    fn invalid_bad_scheme_start() {
        super::parse(".b://abcd").unwrap_err();
    }

    #[test]
    fn invalid_bad_scheme_char() {
        super::parse("a!b://abcd").unwrap_err();
    }
}
