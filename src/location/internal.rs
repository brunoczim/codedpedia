use super::{
    component::{Component, InvalidComponent},
    id::{Id, InvalidId},
    path::{InvalidPath, Path, PathBuf},
};
use std::{error::Error, fmt, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidInternal {
    InvalidPath(InvalidPath),
    InvalidId(InvalidId),
}

impl fmt::Display for InvalidInternal {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPath(error) => fmt::Display::fmt(error, fmtr),
            Self::InvalidId(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl From<InvalidPath> for InvalidInternal {
    fn from(error: InvalidPath) -> Self {
        Self::InvalidPath(error)
    }
}

impl From<InvalidId> for InvalidInternal {
    fn from(error: InvalidId) -> Self {
        Self::InvalidId(error)
    }
}

impl Error for InvalidInternal {}

pub fn parse<'a>(
    input: &'a str,
) -> Result<(&'a Internal, ViewRef<'a>), InvalidInternal> {
    let view =
        match input.char_indices().find(|(_, ch)| *ch == '#').map(|(i, _)| i) {
            Some(i) => {
                let (path_str, id_str) = input.split_at(i);
                let id = Id::parse(id_str)?;
                if path_str == "." {
                    View::Id(id)
                } else {
                    let path = Path::parse(path_str)?;
                    View::PathWithId(path, id)
                }
            },

            None => {
                let path = Path::parse(input)?;
                View::Path(path)
            },
        };

    let internal_loc = Internal::from_ref_unchecked(input);
    Ok((internal_loc, view))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Internal {
    contents: str,
}

impl Internal {
    pub fn parse(input: &str) -> Result<&Self, InvalidInternal> {
        let (internal_loc, _) = parse(input)?;
        Ok(internal_loc)
    }

    pub fn parse_boxed(input: Box<str>) -> Result<Box<Self>, InvalidInternal> {
        parse(&input[..])?;
        Ok(Self::from_box_unchecked(input))
    }

    pub fn raw_contents(&self) -> &str {
        &self.contents
    }

    pub fn into_boxed(&self) -> Box<Self> {
        Self::from_box_unchecked(Box::from(self.raw_contents()))
    }

    pub fn view(&self) -> ViewRef {
        match self
            .contents
            .char_indices()
            .find(|(_, ch)| *ch == '#')
            .map(|(i, _)| i)
        {
            Some(i) => {
                let (path_str, id_str) = self.contents.split_at(i);
                let id = Id::from_ref_unchecked(id_str);
                if path_str == "." {
                    View::Id(id)
                } else {
                    let path = Path::from_ref_unchecked(path_str);
                    View::PathWithId(path, id)
                }
            },

            None => View::Path(Path::from_ref_unchecked(&self.contents)),
        }
    }

    pub(crate) const fn from_ref_unchecked(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    pub(crate) const fn from_box_unchecked(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }
}

impl Clone for Box<Internal> {
    fn clone(&self) -> Self {
        self.into_boxed()
    }
}

impl<'a> From<&'a Internal> for Box<Internal> {
    fn from(reference: &'a Internal) -> Self {
        reference.into_boxed()
    }
}

impl PartialEq<str> for Internal {
    fn eq(&self, other: &str) -> bool {
        Self::parse(other).map_or(false, |other| self == other)
    }
}

impl<'input> TryFrom<&'input str> for &'input Internal {
    type Error = InvalidInternal;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Internal::parse(input)
    }
}

impl TryFrom<Box<str>> for Box<Internal> {
    type Error = InvalidInternal;

    fn try_from(input: Box<str>) -> Result<Self, Self::Error> {
        Internal::parse_boxed(input)
    }
}

impl AsRef<Self> for Internal {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for Internal {
    fn as_ref(&self) -> &str {
        self.raw_contents()
    }
}

impl fmt::Display for Internal {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", self.raw_contents())
    }
}

impl ToOwned for Internal {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        self.into_boxed()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum View<P, I>
where
    P: AsRef<Path>,
    I: AsRef<Id>,
{
    Id(I),
    Path(P),
    PathWithId(P, I),
}

impl<P, I> View<P, I>
where
    P: AsRef<Path>,
    I: AsRef<Id>,
{
    pub fn to_boxed(&self) -> Box<Internal> {
        let internal_str = self.to_string();
        Internal::from_box_unchecked(internal_str.into_boxed_str())
    }

    pub fn path(&self) -> Option<&P> {
        match self {
            Self::Path(path) | Self::PathWithId(path, _) => Some(path),
            Self::Id(_) => None,
        }
    }

    pub fn id(&self) -> Option<&I> {
        match self {
            Self::Id(id) | Self::PathWithId(_, id) => Some(id),
            Self::Path(_) => None,
        }
    }

    pub fn path_mut(&mut self) -> Option<&mut P> {
        match self {
            Self::Path(path) | Self::PathWithId(path, _) => Some(path),
            Self::Id(_) => None,
        }
    }

    pub fn id_mut(&mut self) -> Option<&mut I> {
        match self {
            Self::Id(id) | Self::PathWithId(_, id) => Some(id),
            Self::Path(_) => None,
        }
    }

    pub fn with_path<Q>(self, path: Q) -> View<Q, I>
    where
        Q: AsRef<Path>,
    {
        match self {
            View::Path(_) => View::Path(path),
            View::Id(id) | View::PathWithId(_, id) => {
                View::PathWithId(path, id)
            },
        }
    }

    pub fn with_id<J>(self, id: J) -> View<P, J>
    where
        J: AsRef<Id>,
    {
        match self {
            View::Id(_) => View::Id(id),
            View::Path(path) | View::PathWithId(path, _) => {
                View::PathWithId(path, id)
            },
        }
    }

    pub fn or_insert_path<F>(&mut self, make_path: F) -> &mut P
    where
        F: FnOnce() -> P,
        P: Default,
    {
        if matches!(self, View::Id(_)) {
            let new_path = make_path();

            let View::Id(id) = mem::replace(self, View::Path(P::default()))
            else {
                unreachable!()
            };

            *self = View::PathWithId(new_path, id);
        }

        let (View::Path(path) | View::PathWithId(path, _)) = self else {
            unreachable!()
        };

        path
    }

    pub fn or_insert_id<F>(&mut self, make_id: F) -> &mut I
    where
        F: FnOnce() -> I,
        I: Default,
    {
        if matches!(self, View::Path(_)) {
            let new_id = make_id();

            let View::Path(path) = mem::replace(self, View::Id(I::default()))
            else {
                unreachable!()
            };

            *self = View::PathWithId(path, new_id);
        }

        let (View::Id(id) | View::PathWithId(_, id)) = self else {
            unreachable!()
        };

        id
    }
}

impl<P, I> fmt::Display for View<P, I>
where
    P: AsRef<Path>,
    I: AsRef<Id>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Id(id) => write!(fmtr, ".#{}", id.as_ref()),
            Self::Path(path) => write!(fmtr, "{}", path.as_ref()),
            Self::PathWithId(path, id) => {
                write!(fmtr, "{}#{}", path.as_ref(), id.as_ref())
            },
        }
    }
}

pub type ViewRef<'a> = View<&'a Path, &'a Id>;

impl<'a> TryFrom<&'a str> for ViewRef<'a> {
    type Error = InvalidInternal;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl<'a> ViewRef<'a> {
    pub fn parse(input: &'a str) -> Result<Self, InvalidInternal> {
        let (_, view) = parse(input)?;
        Ok(view)
    }
}

pub type ViewBuf = View<PathBuf, Box<Id>>;

impl ViewBuf {
    pub fn as_path(&self) -> Option<&Path> {
        self.path().map(PathBuf::as_path)
    }

    pub fn pop(&mut self) -> bool {
        self.path_mut().map_or(false, PathBuf::pop)
    }

    pub fn push(&mut self, component: &Component) {
        self.or_insert_path(PathBuf::default).push(component);
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

    pub fn append_str(self, component_str: &str) -> Self {
        self.try_append_str(component_str)
            .expect("attempt to append invalid component in path")
    }
}
