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

impl AsRef<External> for External {
    fn as_ref(&self) -> &External {
        self
    }
}
