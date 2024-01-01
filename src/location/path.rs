use super::component::{Component, InvalidComponent};
use std::{borrow::Borrow, error::Error, fmt, mem, ops::Deref, str};

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
    pub const ROOT: &'static Self = Self::from_ref_unchecked("");

    pub fn new(input: &str) -> Result<&Self, InvalidPath> {
        if !input.is_empty() {
            for component_str in input.split('/') {
                Component::new(component_str)?;
            }
        }

        Ok(Self::from_ref_unchecked(input))
    }

    pub fn new_boxed(input: Box<str>) -> Result<Box<Self>, InvalidPath> {
        Self::new(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn to_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn is_root(&self) -> bool {
        self.contents.is_empty()
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

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }

    pub(crate) fn into_boxed_contents(self: Box<Self>) -> Box<str> {
        unsafe { mem::transmute(self) }
    }
}

impl<'a> Default for &'a Path {
    fn default() -> Self {
        Path::ROOT
    }
}

impl Default for Box<Path> {
    fn default() -> Self {
        Path::ROOT.to_boxed()
    }
}

impl<'a> From<&'a Component> for &'a Path {
    fn from(component: &'a Component) -> Self {
        Path::from_ref_unchecked(component.raw_contents())
    }
}

impl From<Box<Component>> for Box<Path> {
    fn from(component: Box<Component>) -> Self {
        Path::from_box_unchecked(component.into_boxed_contents())
    }
}

impl Clone for Box<Path> {
    fn clone(&self) -> Self {
        self.to_boxed()
    }
}

impl<'a> From<&'a Path> for Box<Path> {
    fn from(reference: &'a Path) -> Self {
        reference.to_boxed()
    }
}

impl PartialEq<str> for Path {
    fn eq(&self, other: &str) -> bool {
        Self::new(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input Path {
    type Error = InvalidPath;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Path::new(input)
    }
}

impl TryFrom<Box<str>> for Box<Path> {
    type Error = InvalidPath;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Path::new_boxed(input)
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

impl AsRef<Path> for Component {
    fn as_ref(&self) -> &Path {
        Path::from_ref_unchecked(self.raw_contents())
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.raw_contents()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> Self::Owned {
        self.to_buf()
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn try_push_str(
        &mut self,
        component_str: &str,
    ) -> Result<(), InvalidComponent> {
        self.push(Component::new(component_str)?);
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

    pub fn append_str(self, component_str: &str) -> Self {
        self.try_append_str(component_str)
            .expect("attempt to append invalid component in path")
    }
}

impl Default for PathBuf {
    fn default() -> Self {
        Self::ROOT
    }
}

impl<'path> From<&'path Path> for PathBuf {
    fn from(path: &'path Path) -> Self {
        path.to_buf()
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        self.as_path()
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

impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        self.raw_contents()
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.as_path()
    }
}

impl From<PathBuf> for Box<Path> {
    fn from(buf: PathBuf) -> Self {
        Path::from_box_unchecked(buf.contents.into())
    }
}

#[cfg(test)]
mod test {
    use crate::location::component::Component;

    use super::Path;

    #[test]
    fn valid_root() {
        let path0 = Path::new("").unwrap();
        let path1 = Path::ROOT;

        assert_eq!(path0, path1);
        assert_eq!(path0.raw_contents(), "");
        assert_eq!(path1.raw_contents(), "");
    }

    #[test]
    fn valid_single_alphanumeric() {
        let path = Path::new("hell0").unwrap();
        assert_eq!(path.raw_contents(), "hell0");
    }

    #[test]
    fn valid_slug() {
        let path = Path::new("hello-world/foo-bar").unwrap();
        assert_eq!(path.raw_contents(), "hello-world/foo-bar");
    }

    #[test]
    fn valid_with_spaces_and_punct() {
        let path = Path::new("Hello, world!/test/done").unwrap();
        assert_eq!(path.raw_contents(), "Hello, world!/test/done");
    }

    #[test]
    fn invalid_hash() {
        Path::new("ha#he").unwrap_err();
    }

    #[test]
    fn iter_root() {
        let expected: Vec<&Component> = Vec::new();
        let actual = Path::ROOT.into_iter().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn iter_single() {
        let expected: Vec<&Component> = vec![Component::new("bl0g").unwrap()];
        let actual = Path::new("bl0g").unwrap().into_iter().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn iter_two() {
        let expected: Vec<&Component> = vec![
            Component::new("bl0g").unwrap(),
            Component::new("new-post").unwrap(),
        ];
        let actual =
            Path::new("bl0g/new-post").unwrap().into_iter().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn iter_three() {
        let expected: Vec<&Component> = vec![
            Component::new("bl0g").unwrap(),
            Component::new("new-post").unwrap(),
            Component::new("actual").unwrap(),
        ];
        let actual = Path::new("bl0g/new-post/actual")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }
}
