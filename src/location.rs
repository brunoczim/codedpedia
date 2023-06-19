//! This module provides location, paths, Urls.

use crate::{
    component::{Component, InlineComponent},
    render::{Context, Format, Html, Markdown, Render, Renderer, Text},
};
use percent_encoding::{percent_encode, CONTROLS};
use std::{
    error::Error,
    fmt::{self, Write},
    path::PathBuf,
    str,
};
use url::Url;

/// A location of a page, either internal or external.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    /// An external page (or an internal page encoded as an external Url).
    Url(Url),
    /// An internal location.
    Internal(InternalLoc),
}

impl Default for Location {
    fn default() -> Self {
        Self::Internal(InternalLoc::default())
    }
}

impl From<InternalPath> for Location {
    fn from(path: InternalPath) -> Self {
        Location::Internal(InternalLoc::from(path))
    }
}

impl From<InternalLoc> for Location {
    fn from(loc: InternalLoc) -> Self {
        Location::Internal(loc)
    }
}

impl From<Url> for Location {
    fn from(url: Url) -> Self {
        Location::Url(url)
    }
}

impl Location {
    /// Parses a URL and yields a location.
    ///
    /// # Panics
    ///
    /// Panics if the url string is invalid.
    pub fn url<S>(contents: S) -> Self
    where
        S: AsRef<str>,
    {
        Location::Url(Url::parse(contents.as_ref()).expect("bad URL"))
    }

    /// Parses an internal location but returns a generic location.
    ///
    /// # Panics
    ///
    /// Panics if the location string is invalid.
    pub fn internal<S>(contents: S) -> Self
    where
        S: AsRef<str>,
    {
        Location::Internal(
            InternalLoc::parse(contents).expect("bad internal location"),
        )
    }
}

impl Component for Location {
    type Kind = InlineComponent;
}

impl Render<Html> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Location::Url(url) => write!(renderer, "{}", url),
            Location::Internal(int) => int.render(renderer, ctx),
        }
    }
}

impl Render<Markdown> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Location::Url(url) => write!(renderer, "{}", url),
            Location::Internal(int) => int.render(renderer, ctx),
        }
    }
}

impl Render<Text> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Location::Url(url) => write!(renderer, "{}", url),
            Location::Internal(int) => int.render(renderer, ctx),
        }
    }
}

/// An internal path, without any ID. Always absolute (with the root pointing to
/// the root of the encyclopedia).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalPath {
    /// Fragments of the path (i.e. each piece, each element).
    pub fragments: Vec<Fragment>,
}

impl InternalPath {
    /// Parser the internal path. Fragments separated by "/".
    pub fn parse<S>(string: S) -> Result<Self, InvalidFragment>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref();
        let mut this = Self { fragments: Vec::new() };

        if string.len() > 0 {
            for fragment in string.split('/') {
                this.fragments.push(Fragment::new(fragment)?);
            }
        }

        Ok(this)
    }

    /// Path to the root of the encyclopedia.
    pub fn root() -> Self {
        Self { fragments: Vec::new() }
    }

    /// Tests if this path leads to the root.
    pub fn is_root(&self) -> bool {
        self.fragments.len() == 0
    }

    /// Counts the directory depth.
    pub fn dir_depth(&self) -> usize {
        self.fragments.len().saturating_sub(1)
    }

    /// Creates an OS path buffer.
    pub fn to_fs_path(&self) -> PathBuf {
        PathBuf::from(format!("{}", self))
    }

    /// Appends a fragment (a piece) to the end of this path. Returns the
    /// modified path.
    pub fn append(mut self, fragment: Fragment) -> Self {
        self.fragments.push(fragment);
        self
    }

    /// Compares two locations taking "index" into account and ignoring its
    /// presence.
    pub fn eq_index(&self, other: &Self) -> bool {
        if self.fragments.len() == other.fragments.len() + 1 {
            let last_index = self.fragments.len() - 1;
            self.fragments[last_index].as_str() == "index.html"
                && self.fragments[.. last_index] == other.fragments
        } else if other.fragments.len() == self.fragments.len() + 1 {
            let last_index = other.fragments.len() - 1;
            other.fragments[last_index].as_str() == "index.html"
                && other.fragments[.. last_index] == self.fragments
        } else {
            self == other
        }
    }

    fn render_as_url<W>(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<<Self as Component>::Kind>,
    ) -> fmt::Result
    where
        W: Format + ?Sized,
        String: Render<W>,
    {
        if !self.eq_index(ctx.location()) {
            for _ in 0 .. ctx.location().dir_depth() {
                renderer.write_str("../")?;
            }
            let encoded = percent_encode(self.to_string().as_bytes(), CONTROLS)
                .collect::<String>();
            encoded.render(renderer, ctx)?;
        }
        Ok(())
    }
}

impl Default for InternalPath {
    fn default() -> Self {
        Self::root()
    }
}

impl fmt::Display for InternalPath {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        for fragment in &self.fragments {
            if first {
                first = false;
            } else {
                fmt.write_str("/")?;
            }
            write!(fmt, "{}", fragment)?;
        }

        Ok(())
    }
}

impl Component for InternalPath {
    type Kind = InlineComponent;
}

impl Render<Html> for InternalPath {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.render_as_url(renderer, ctx)?;
        Ok(())
    }
}

impl Render<Markdown> for InternalPath {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.render_as_url(renderer, ctx)?;
        Ok(())
    }
}

impl Render<Text> for InternalPath {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "{}", self)
    }
}

/// Error yielded when an internal location was attempted to be constructed from
/// an invalid string.
#[derive(Debug, Clone)]
pub enum InvalidInternalLoc {
    /// The ID part is invalid.
    Id(InvalidId),
    /// The fragment part is invalid.
    Fragment(InvalidFragment),
}

impl fmt::Display for InvalidInternalLoc {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Id(error) => write!(fmtr, "{}", error),
            Self::Fragment(error) => write!(fmtr, "{}", error),
        }
    }
}

impl From<InvalidId> for InvalidInternalLoc {
    fn from(error: InvalidId) -> Self {
        Self::Id(error)
    }
}

impl From<InvalidFragment> for InvalidInternalLoc {
    fn from(error: InvalidFragment) -> Self {
        Self::Fragment(error)
    }
}

impl Error for InvalidInternalLoc {}

/// A location to an internal page, with optional ID. Always absolute (with the
/// root pointing to the root of the encyclopedia).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InternalLoc {
    /// Path to the document.
    pub path: InternalPath,
    /// ID of the section or specific object inside of the document.
    pub id: Option<Id>,
}

impl From<InternalPath> for InternalLoc {
    fn from(path: InternalPath) -> Self {
        Self { path, id: None }
    }
}

impl InternalLoc {
    /// Parses247     } an internal location. Path fragments separated by "/",
    /// ID appended to the end with "#" between the path and the ID, if any
    /// ID at all.
    pub fn parse<S>(string: S) -> Result<Self, InvalidInternalLoc>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref();
        let hash = string
            .as_bytes()
            .iter()
            .rposition(|&ch| ch == b'#')
            .unwrap_or(string.len());

        Ok(Self {
            path: InternalPath::parse(&string[.. hash])?,
            id: if hash == string.len() {
                None
            } else {
                Some(Id::new(&string[hash + 1 ..])?)
            },
        })
    }

    fn render_as_url<W>(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<<Self as Component>::Kind>,
    ) -> fmt::Result
    where
        W: Format + ?Sized,
        InternalPath: Render<W>,
    {
        self.path.render(renderer, ctx)?;
        if let Some(id) = &self.id {
            renderer.write_str("#")?;
            id.render(renderer, ctx)?;
        }
        Ok(())
    }
}

impl fmt::Display for InternalLoc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", &self.path)?;

        if let Some(id) = &self.id {
            write!(fmt, "#{}", id)?;
        }

        Ok(())
    }
}

impl Component for InternalLoc {
    type Kind = InlineComponent;
}

impl Render<Html> for InternalLoc {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.render_as_url(renderer, ctx)
    }
}

impl Render<Markdown> for InternalLoc {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.render_as_url(renderer, ctx)
    }
}

impl Render<Text> for InternalLoc {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.render_as_url(renderer, ctx)
    }
}

/// Error when an invalid ID string is given to be parsed.
#[derive(Debug, Clone)]
pub struct InvalidId;

impl fmt::Display for InvalidId {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("Invalid ID string")
    }
}

impl Error for InvalidId {}

/// An ID of a location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    contents: Box<str>,
}

impl Id {
    /// Creates an ID from the desired string contents. The string can only
    /// contain alphanumeric characters or '_' or '-'.
    pub fn new<S>(contents: S) -> Result<Self, InvalidId>
    where
        S: AsRef<str> + Into<Box<str>>,
    {
        let mut iter = contents.as_ref().as_bytes().iter();

        iter.next().filter(|ch| ch.is_ascii_alphabetic()).ok_or(InvalidId)?;

        for &ch in iter {
            if !ch.is_ascii_alphanumeric() && ch != b'_' && ch != b'-' {
                Err(InvalidId)?;
            }
        }

        Ok(Self { contents: contents.into() })
    }

    /// The string contents of this ID.
    pub fn as_str(&self) -> &str {
        &self.contents
    }
}

impl fmt::Display for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_str())
    }
}

impl Component for Id {
    type Kind = InlineComponent;
}

impl<W> Render<W> for Id
where
    W: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        write!(renderer, "{}", self)
    }
}

/// Error when an invalid fragment (piece of a path) string is given to be
/// parsed.
#[derive(Debug, Clone)]
pub struct InvalidFragment;

impl fmt::Display for InvalidFragment {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str("Invalid location fragment string")
    }
}

impl Error for InvalidFragment {}

/// A fragment of a path, that is, a piece, an element of it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment {
    contents: Box<str>,
}

impl Fragment {
    /// Creates a fragment from the desired string contents. The string cannot
    /// contain '/' or '#', it cannot be empty or composed of only "." or
    /// ".." as well.
    pub fn new<S>(contents: S) -> Result<Self, InvalidFragment>
    where
        S: AsRef<str> + Into<Box<str>>,
    {
        if let "" | "." | ".." = contents.as_ref() {
            Err(InvalidFragment)?;
        }

        for ch in contents.as_ref().bytes() {
            if let b'/' | b'#' = ch {
                Err(InvalidFragment)?;
            }
        }

        Ok(Self { contents: contents.into() })
    }

    /// The string contents of this fragment.
    pub fn as_str(&self) -> &str {
        &self.contents
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::InternalPath;

    #[test]
    fn eq_index() {
        let left = InternalPath::parse("langs/div-prt/phonology").unwrap();
        let right =
            InternalPath::parse("langs/div-prt/phonology/index.html").unwrap();
        assert!(left.eq_index(&right));
        assert!(right.eq_index(&left));
    }
}
