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
    pub fn new(input: &str) -> Result<&Self, InvalidComponent> {
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

    pub fn new_boxed(input: Box<str>) -> Result<Box<Self>, InvalidComponent> {
        Self::new(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn to_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
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

    pub(crate) fn into_boxed_contents(self: Box<Self>) -> Box<str> {
        unsafe { mem::transmute(self) }
    }
}

impl Clone for Box<Component> {
    fn clone(&self) -> Self {
        self.to_boxed()
    }
}

impl<'a> From<&'a Component> for Box<Component> {
    fn from(reference: &'a Component) -> Self {
        reference.to_boxed()
    }
}

impl PartialEq<str> for Component {
    fn eq(&self, other: &str) -> bool {
        Self::new(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input Component {
    type Error = InvalidComponent;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Component::new(input)
    }
}

impl TryFrom<Box<str>> for Box<Component> {
    type Error = InvalidComponent;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Component::new_boxed(input)
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

impl ToOwned for Component {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        self.to_boxed()
    }
}

#[cfg(test)]
mod test {
    use super::Component;

    #[test]
    fn valid_alphanumeric() {
        let component = Component::new("hell0").unwrap();
        assert_eq!(component.raw_contents(), "hell0");
    }

    #[test]
    fn valid_slug() {
        let component = Component::new("hello-world").unwrap();
        assert_eq!(component.raw_contents(), "hello-world");
    }

    #[test]
    fn valid_with_spaces_and_punct() {
        let component = Component::new("Hello, world!").unwrap();
        assert_eq!(component.raw_contents(), "Hello, world!");
    }

    #[test]
    fn invalid_bar() {
        Component::new("ha/he").unwrap_err();
    }

    #[test]
    fn invalid_hash() {
        Component::new("ha#he").unwrap_err();
    }
}
