pub mod buf;
pub mod parse;
pub mod id;
pub mod fragment;
pub mod path;
pub mod internal;
pub mod external;
pub mod general;
pub mod unify;

/*
pub trait CompleteLocation: Clone + fmt::Debug + Eq {
    type ParseError: Error;

    fn parse(input: &str) -> Result<Self, Self::ParseError>;

    fn into_location(self) -> Location;

    fn eq_with_ctx(&self, location: &Location, page_path: &PagePath) -> bool;

    fn render_raw<W>(&self, page_path: &Path, writer: &mut W) -> fmt::Result
    where
        W: Write;
}

pub trait AsRefType<'this, B> {
    fn as_ref_type(&'this self) -> B;
}

pub trait IntoOwnedType {
    type Owned;

    fn into_owned_type(self) -> Self::Owned;
}

#[derive(Debug, Clone)]
pub enum Location<P = Path, I = Id> {
    Internal(Internal<P, I>),
    External(Url),
}

pub type LocationRef<'path, 'id> =
    Location<PathRef<'path>, BorrowedId<'id>>;

pub type LocationFragmentsRef<'path, 'fragment, 'id> =
    Location<PathFragmentsRef<'path, 'fragment>, BorrowedId<'id>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Internal<P = Path, I = Id> {
    Path(P),
    Id(I),
    PathWithId(P, I),
}

pub type InternalRef<'path, 'id> =
    Internal<PathRef<'path>, BorrowedId<'id>>;

pub type InternalFragmentsRef<'path, 'fragment, 'id> =
    Internal<PathFragmentsRef<'path, 'fragment>, BorrowedId<'id>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path<FS = Vec<Fragment>> {
    pub fragments: FS,
}

pub type PathRef<'fragments, F = Fragment> = Path<&'fragments [F]>;

pub type PathFragmentsRef<'fragments, 'fragmment> =
    PathRef<'fragments, FragmentRef<'fragmment>>;

impl<'this, FS> AsRefType<'this, PathRef<'this, Fragment>> for Path<FS>
where
    FS: AsRef<[Fragment]>,
{
    fn as_ref_type(&'this self) -> PathRef<'this> {
        Path { fragments: self.fragments.as_ref() }
    }
}

impl<FS> IntoOwnedType for Path<FS>
where
    FS: Into<Vec<Fragment>>,
{
    type Owned = Path<Vec<Fragment>>;

    fn into_owned_type(self) -> Self::Owned {
        Path { fragments: self.fragments.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment<S = Box<str>> {
    contents: S,
}

pub type FragmentRef<'contents> = Fragment<&'contents str>;

impl<'this, S> AsRefType<'this, FragmentRef<'this>> for Fragment<S>
where
    S: AsRef<str>,
{
    fn as_ref_type(&'this self) -> FragmentRef<'this> {
        Fragment { contents: self.contents.as_ref() }
    }
}

impl<S> IntoOwnedType for Fragment<S>
where
    S: Into<Box<str>>,
{
    type Owned = Fragment;

    fn into_owned_type(self) -> Self::Owned {
        Fragment { contents: self.contents.into() }
    }
}

impl<S> AsRef<Fragment<S>> for Fragment<S> {
    fn as_ref(&self) -> &Fragment<S> {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<S = Box<str>> {
    contents: S,
}

pub type BorrowedId<'contents> = Id<&'contents str>;

impl<'this, S> AsRefType<'this, BorrowedId<'this>> for Id<S>
where
    S: AsRef<str>,
{
    fn as_ref_type(&'this self) -> BorrowedId<'this> {
        Id { contents: self.contents.as_ref() }
    }
}

impl<S> IntoOwnedType for Id<S>
where
    S: Into<Box<str>>,
{
    type Owned = Id;

    fn into_owned_type(self) -> Self::Owned {
        Id { contents: self.contents.into() }
    }
}

impl<S> AsRef<Id<S>> for Id<S> {
    fn as_ref(&self) -> &Id<S> {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidPath {
    InvalidFragment(InvalidFragment),
}

impl From<InvalidFragment> for InvalidPath {
    fn from(error: InvalidFragment) -> Self {
        Self::InvalidFragment(error)
    }
}

impl fmt::Display for InvalidPath {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFragment(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidPath {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidFragment(error) => Some(error),
        }
    }
}

impl Path {
    pub const ROOT: Self = Self { fragments: Vec::new() };
}

impl CompleteLocation for Path {
    type ParseError = InvalidPath;

    fn parse(input: &str) -> Result<Self, Self::ParseError> {
        let mut this = Self::ROOT;

        if !input.is_empty() {
            for fragment_str in input.split('/') {
                this.fragments.push(Fragment::parse(fragment_str)?);
            }
        }

        Ok(this)
    }

    fn into_location(self) -> Location {
        Location::Internal(Internal::Path(self))
    }

    fn render_raw<W>(&self, page_path: &Path, writer: &mut W) -> fmt::Result
    where
        W: Write,
    {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidFragment {
    Empty,
    CurrentDir,
    ParentDir,
    Bar,
    Hash,
}

impl fmt::Display for InvalidFragment {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmtr, "Fragment cannot be empty"),
            Self::CurrentDir => {
                write!(fmtr, "Fraagment cannot reference current directory")
            },
            Self::ParentDir => {
                write!(fmtr, "Fragment cannot reference parent directory")
            },
            Self::Bar => write!(fmtr, "Fragment cannot contain a bar '/'"),
            Self::Hash => write!(fmtr, "Fragment cannot contain a hash '#'"),
        }
    }
}

impl Error for InvalidFragment {}

impl CompleteLocation for Fragment {
    type ParseError = InvalidFragment;

    fn parse(input: &str) -> Result<Self, Self::ParseError> {
        if input.is_empty() {
            Err(InvalidFragment::Empty)?;
        }
        if input == "." {
            Err(InvalidFragment::CurrentDir)?;
        }
        if input == ".." {
            Err(InvalidFragment::ParentDir)?;
        }

        for ch in input.chars() {
            if ch == '/' {
                Err(InvalidFragment::Bar)?;
            }
            if ch == '#' {
                Err(InvalidFragment::Hash)?;
            }
        }

        Ok(Self { contents: input.into() })
    }

    fn into_location(self) -> Location {
        Location::Internal(Internal::Path(Path { fragments: vec![self] }))
    }

    fn render_raw<W>(&self, page_path: &Path, writer: &mut W) -> fmt::Result
    where
        W: Write,
    {
        if let Some(first) = page_path.fragments.first() {
            if first == self {
                for _ in 1 .. page_path.fragments.len() {
                    write!(writer, "../")?;
                }
            } else {
                for _ in 0 .. page_path.fragments.len() {
                    write!(writer, "../")?;
                }
                write!(writer, "{}", self.contents)?;
            }
        } else {
            write!(writer, "{}", self.contents)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidId {
    Empty,
    InvalidStart(char),
    InvalidChar(char),
}

impl fmt::Display for InvalidId {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(fmtr, "Id cannot be empty"),
            Self::InvalidStart(ch) => write!(
                fmtr,
                "Id must start with an ASCII letter, found {:?}",
                ch
            ),
            Self::InvalidChar(ch) => write!(
                fmtr,
                "Id must contain only ASCII letters, digits, '_' or '-', \
                 found {:?}",
                ch
            ),
        }
    }
}

impl Error for InvalidId {}

impl CompleteLocation for Id {
    type ParseError = InvalidId;

    fn parse(input: &str) -> Result<Self, Self::ParseError> {
        let mut iter = input.chars();

        let ch = iter.next().ok_or(InvalidId::Empty)?;
        if !ch.is_ascii_alphabetic() {
            Err(InvalidId::InvalidStart(ch))?;
        }

        for ch in iter {
            if !ch.is_ascii_alphanumeric() && ch != '_' && ch != '-' {
                Err(InvalidId::InvalidChar(ch))?;
            }
        }

        Ok(Self { contents: input.into() })
    }

    fn into_location(self) -> Location {
        Location::Internal(Internal::Id(self))
    }

    fn eq_with_ctx(&self, location: &Location, page_path: &PagePath) -> bool {
        match location {
            Location::External(_) => false,
            Location::Internal(Internal::Path(_)) => false,
            Location::Internal(Internal::Id(id)) => id == self,
            Location::Internal(Internal::PathWithId(path, id)) => {
                id == self && path.eq_with_ctx(page_path, page_path)
            },
        }
    }

    fn render_raw<W>(&self, _page_path: &Path, writer: &mut W) -> fmt::Result
    where
        W: Write,
    {
        write!(writer, "#{}", self.contents)
    }
}
*/
