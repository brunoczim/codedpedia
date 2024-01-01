use std::{error::Error, fmt, mem};

use super::{
    component::Component,
    external::{External, InvalidExternal},
    id::Id,
    internal::{Internal, InvalidInternal},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidLocation {
    InvalidInternal(InvalidInternal),
    InvalidExternal(InvalidExternal),
}

impl From<InvalidExternal> for InvalidLocation {
    fn from(error: InvalidExternal) -> Self {
        Self::InvalidExternal(error)
    }
}

impl From<InvalidInternal> for InvalidLocation {
    fn from(error: InvalidInternal) -> Self {
        Self::InvalidInternal(error)
    }
}

impl fmt::Display for InvalidLocation {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInternal(error) => fmt::Display::fmt(error, fmtr),
            Self::InvalidExternal(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidLocation {}

pub fn parse<'a>(
    input: &'a str,
) -> Result<(&'a Location, ViewRef<'a>), InvalidLocation> {
    let view = if input.contains("://") {
        let external = External::new(input)?;
        ViewRef::External(external)
    } else {
        let internal = Internal::new(input)?;
        ViewRef::Internal(internal)
    };
    let location = Location::from_ref_unchecked(input);
    Ok((location, view))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    contents: str,
}

impl Location {
    pub fn new(input: &str) -> Result<&Self, InvalidLocation> {
        let (external_loc, _) = parse(input)?;
        Ok(external_loc)
    }

    pub fn new_boxed(input: Box<str>) -> Result<Box<Self>, InvalidLocation> {
        Self::new(input.as_ref())?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn from_curr_page_id(id: &Id) -> Box<Self> {
        Box::<Self>::from(Internal::from_curr_page_id(id))
    }

    pub fn to_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn view(&self) -> ViewRef {
        if self.raw_contents().contains("://") {
            let external = External::from_ref_unchecked(self.raw_contents());
            ViewRef::External(external)
        } else {
            let internal = Internal::from_ref_unchecked(self.raw_contents());
            ViewRef::Internal(internal)
        }
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }

    #[allow(dead_code)]
    pub(crate) fn into_boxed_contents(self: Box<Self>) -> Box<str> {
        unsafe { mem::transmute(self) }
    }
}

impl<'a> Default for &'a Location {
    fn default() -> Self {
        Self::from(Path::ROOT)
    }
}

impl Default for Box<Location> {
    fn default() -> Self {
        Self::from(Path::ROOT.to_boxed())
    }
}

impl<'a> From<&'a Component> for &'a Location {
    fn from(component: &'a Component) -> Self {
        Self::from(<&Internal>::from(component))
    }
}

impl<'a> From<&'a Path> for &'a Location {
    fn from(path: &'a Path) -> Self {
        Self::from(<&Internal>::from(path))
    }
}

impl<'a> From<&'a Internal> for &'a Location {
    fn from(internal_loc: &'a Internal) -> Self {
        Location::from_ref_unchecked(internal_loc.raw_contents())
    }
}

impl<'a> From<&'a External> for &'a Location {
    fn from(external_loc: &'a External) -> Self {
        Location::from_ref_unchecked(external_loc.raw_contents())
    }
}

impl From<Box<Component>> for Box<Location> {
    fn from(component: Box<Component>) -> Self {
        Self::from(Box::<Internal>::from(component))
    }
}

impl From<Box<Path>> for Box<Location> {
    fn from(path: Box<Path>) -> Self {
        Self::from(Box::<Internal>::from(path))
    }
}

impl From<Box<Internal>> for Box<Location> {
    fn from(internal_loc: Box<Internal>) -> Self {
        Location::from_box_unchecked(internal_loc.into_boxed_contents())
    }
}

impl From<Box<External>> for Box<Location> {
    fn from(external_loc: Box<External>) -> Self {
        Location::from_box_unchecked(external_loc.into_boxed_contents())
    }
}

impl Clone for Box<Location> {
    fn clone(&self) -> Self {
        self.to_boxed()
    }
}

impl<'a> From<&'a Location> for Box<Location> {
    fn from(reference: &'a Location) -> Self {
        reference.to_boxed()
    }
}

impl PartialEq<str> for Location {
    fn eq(&self, other: &str) -> bool {
        Self::new(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input Location {
    type Error = InvalidLocation;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Location::new(input)
    }
}

impl TryFrom<Box<str>> for Box<Location> {
    type Error = InvalidLocation;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Location::new_boxed(input)
    }
}

impl AsRef<Location> for Location {
    fn as_ref(&self) -> &Location {
        self
    }
}

impl AsRef<Location> for Component {
    fn as_ref(&self) -> &Location {
        Location::from_ref_unchecked(self.raw_contents())
    }
}

impl AsRef<Location> for Path {
    fn as_ref(&self) -> &Location {
        Location::from_ref_unchecked(self.raw_contents())
    }
}

impl AsRef<Location> for Internal {
    fn as_ref(&self) -> &Location {
        Location::from_ref_unchecked(self.raw_contents())
    }
}

impl AsRef<Location> for External {
    fn as_ref(&self) -> &Location {
        Location::from_ref_unchecked(self.raw_contents())
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl ToOwned for Location {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        self.to_boxed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ViewRef<'a> {
    Internal(&'a Internal),
    External(&'a External),
}

impl<'a> TryFrom<&'a str> for ViewRef<'a> {
    type Error = InvalidLocation;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> ViewRef<'a> {
    pub fn parse(input: &'a str) -> Result<Self, InvalidLocation> {
        let (_, view) = parse(input)?;
        Ok(view)
    }

    pub fn to_boxed(&self) -> Box<Location> {
        let location_str = self.to_string();
        Location::from_box_unchecked(location_str.into_boxed_str())
    }
}

impl<'a> fmt::Display for ViewRef<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal(internal) => write!(fmtr, "{}", internal),
            Self::External(external) => write!(fmtr, "{}", external),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::location::{
        external::External,
        general::ViewRef,
        internal::Internal,
    };

    #[test]
    fn valid_internal() {
        let (location, view) = super::parse("blog/page#id").unwrap();
        assert_eq!(location.raw_contents(), "blog/page#id");
        assert_eq!(
            view,
            ViewRef::Internal(Internal::new("blog/page#id").unwrap()),
        );
    }

    #[test]
    fn valid_external() {
        let (location, view) = super::parse("https://duckduckgo.com/").unwrap();
        assert_eq!(location.raw_contents(), "https://duckduckgo.com/");
        assert_eq!(
            view,
            ViewRef::External(
                External::new("https://duckduckgo.com/").unwrap()
            ),
        );
    }

    #[test]
    fn invalid_internal() {
        super::parse("../page#id").unwrap_err();
    }

    #[test]
    fn invalid_external() {
        super::parse("://page").unwrap_err();
    }
}
