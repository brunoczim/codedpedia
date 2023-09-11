use super::{
    buf::{DefaultOwnedStr, TextBuf},
    fragment::{Fragment, FragmentRef, InvalidFragment},
    parse::Parse,
};
use std::{error::Error, fmt, rc::Rc, sync::Arc};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path<FS = Vec<Fragment>> {
    pub fragments: FS,
}

pub type PathRef<'path, F = Fragment> = Path<&'path [F]>;

pub type PathRefFragmentsRef<'path, 'frag> = PathRef<'path, FragmentRef<'frag>>;

pub type PathFragmentsRef<'frag> = Path<Vec<FragmentRef<'frag>>>;

impl<F> Path<Vec<Fragment<F>>>
where
    F: TextBuf,
{
    pub const ROOT: Self = Path { fragments: Vec::new() };
}

impl<F> Path<[Fragment<F>; 0]>
where
    F: TextBuf,
{
    pub const ROOT: Self = Path { fragments: [] };
}

impl<'path, F> Path<&'path [Fragment<F>]>
where
    F: TextBuf,
{
    pub const ROOT: Self = Path { fragments: &[] };
}

impl<'input> Parse<'input> for Path<Vec<FragmentRef<'input>>> {
    type Error = InvalidPath;

    fn parse(input: &'input str) -> Result<Self, Self::Error> {
        let mut this = Self::ROOT;

        if !input.is_empty() {
            for fragment_str in input.split('/') {
                this.fragments.push(Fragment::parse(fragment_str)?);
            }
        }

        Ok(this)
    }
}

pub trait PathFragments {
    type FragmentBuf: TextBuf;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>];

    fn into_fragment_vec(self) -> Vec<Fragment>;
}

impl PathFragments for Vec<Fragment> {
    type FragmentBuf = DefaultOwnedStr;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self
    }
}

impl<'frag> PathFragments for Vec<FragmentRef<'frag>> {
    type FragmentBuf = &'frag str;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}

impl PathFragments for Vec<Fragment<Box<str>>> {
    type FragmentBuf = Box<str>;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}

impl PathFragments for Vec<Fragment<String>> {
    type FragmentBuf = String;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}

impl PathFragments for Vec<Fragment<Rc<str>>> {
    type FragmentBuf = Rc<str>;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}

impl PathFragments for Vec<Fragment<Arc<str>>> {
    type FragmentBuf = Arc<str>;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}

impl<'frag, F> PathFragments for &'frag [Fragment<F>]
where
    F: TextBuf,
{
    type FragmentBuf = F;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        self
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter()
            .map(|fragment| fragment.clone().into_default_owned())
            .collect()
    }
}

impl<F, const N: usize> PathFragments for [Fragment<F>; N]
where
    F: TextBuf,
{
    type FragmentBuf = F;

    fn as_fragments(&self) -> &[Fragment<Self::FragmentBuf>] {
        &self[..]
    }

    fn into_fragment_vec(self) -> Vec<Fragment> {
        self.into_iter().map(|fragment| fragment.into_default_owned()).collect()
    }
}
