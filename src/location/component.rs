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

impl AsRef<Self> for Component {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for Component {
    fn as_ref(&self) -> &str {
        self.raw_contents()
    }
}

impl fmt::Display for Component {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

#[cfg(test)]
mod test {
    use super::Component;

    #[test]
    fn valid_alphanumeric() {
        let component = Component::parse("hell0").unwrap();
        assert_eq!(component.raw_contents(), "hell0");
    }

    #[test]
    fn valid_slug() {
        let component = Component::parse("hello-world").unwrap();
        assert_eq!(component.raw_contents(), "hello-world");
    }

    #[test]
    fn valid_with_spaces_and_punct() {
        let component = Component::parse("Hello, world!").unwrap();
        assert_eq!(component.raw_contents(), "Hello, world!");
    }

    #[test]
    fn invalid_bar() {
        Component::parse("ha/he").unwrap_err();
    }

    #[test]
    fn invalid_hash() {
        Component::parse("ha#he").unwrap_err();
    }
}
