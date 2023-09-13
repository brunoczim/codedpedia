use super::component::{Component, InvalidComponent};
use std::{error::Error, fmt, mem, ops::Deref, str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidPath {
    InvalidComponent(InvalidComponent),
}

impl From<InvalidComponent> for InvalidPath {
    fn from(error: InvalidComponent) -> Self {
        Self::InvalidComponent(error)
    }
}

impl fmt::Display for InvalidPath {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidComponent(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidPath {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidComponent(error) => Some(error),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Path {
    contents: str,
}

impl Path {
    pub const ROOT: &Self = Self::from_ref_unchecked("");

    pub fn parse(input: &str) -> Result<&Self, InvalidPath> {
        if !input.is_empty() {
            for component_str in input.split('/') {
                Component::parse(component_str)?;
            }
        }

        Ok(Self::from_ref_unchecked(input))
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidPath> {
        Self::parse(input.as_ref())?;
        Ok(Self::from_box_uncheckedd(input))
    }

    pub fn is_root(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn components(&self) -> Components {
        let mut inner = self.contents.split('/');
        if self.contents.is_empty() {
            inner.next();
        }
        Components { inner }
    }

    pub fn popped(&self) -> Option<&Self> {
        let (popped_str, _) = self.contents.rsplit_once('/')?;
        Some(Self::from_ref_unchecked(popped_str))
    }

    pub fn to_buf(&self) -> PathBuf {
        PathBuf { contents: String::from(&self.contents) }
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_uncheckedd(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl<'input> TryFrom<&'input str> for &'input Path {
    type Error = InvalidPath;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Path::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Path> {
    type Error = InvalidPath;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Path::parse_boxed(input)
    }
}

impl<'path> IntoIterator for &'path Path {
    type Item = &'path Component;
    type IntoIter = Components<'path>;

    fn into_iter(self) -> Self::IntoIter {
        self.components()
    }
}

impl AsRef<Self> for Path {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Components<'path> {
    inner: str::Split<'path, char>,
}

impl<'path> Iterator for Components<'path> {
    type Item = &'path Component;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Component::from_ref_unchecked)
    }
}

impl<'path> DoubleEndedIterator for Components<'path> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(Component::from_ref_unchecked)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathBuf {
    contents: String,
}

impl PathBuf {
    pub const ROOT: Self = Self { contents: String::new() };

    pub fn as_path(&self) -> &Path {
        Path::from_ref_unchecked(&self.contents)
    }

    pub fn pop(&mut self) -> bool {
        if self.is_root() {
            false
        } else {
            let i = self
                .contents
                .char_indices()
                .rfind(|(_, ch)| *ch == '/')
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.contents.replace_range(i .., "");
            true
        }
    }

    pub fn push(&mut self, component: &Component) {
        if !self.is_root() {
            self.contents.push('/');
        }
        self.contents.push_str(component.raw_contents());
    }

    pub fn push_str(&mut self, component_str: &str) {
        self.try_push_str(component_str)
            .expect("could not make a path component of a string")
    }

    pub fn try_push_str(
        &mut self,
        component_str: &str,
    ) -> Result<(), InvalidComponent> {
        self.push(Component::parse(component_str)?);
        Ok(())
    }

    pub fn append(mut self, component: &Component) -> Self {
        self.push(component);
        self
    }

    pub fn try_append_str(
        mut self,
        component_str: &str,
    ) -> Result<Self, InvalidComponent> {
        self.try_push_str(component_str)?;
        Ok(self)
    }

    pub fn append_str(mut self, component_str: &str) -> Self {
        self.push_str(component_str);
        self
    }
}

impl<'path> From<&'path Path> for PathBuf {
    fn from(path: &'path Path) -> Self {
        path.to_buf()
    }
}

impl AsRef<Self> for PathBuf {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}
