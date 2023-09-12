use super::component::{AsComponent, Component, InvalidComponent};
use std::{
    error::Error,
    fmt,
    mem::{self, MaybeUninit},
    rc::Rc,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidPath {
    PathToBig { buf_len: usize },
    PathToSmall { buf_len: usize },
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
            Self::PathToBig { buf_len } => {
                write!(
                    fmtr,
                    "path is too big for the given buffer of exact length {}",
                    buf_len
                )
            },
            Self::PathToSmall { buf_len } => {
                write!(
                    fmtr,
                    "path is too small for the given buffer of exact length {}",
                    buf_len
                )
            },
            Self::InvalidComponent(error) => fmt::Display::fmt(error, fmtr),
        }
    }
}

impl Error for InvalidPath {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PathToBig { buf_len: _ } => None,
            Self::PathToSmall { buf_len: _ } => None,
            Self::InvalidComponent(error) => Some(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathBuf<C>
where
    C: AsComponent,
{
    pub components: Vec<C>,
}

impl<C> PathBuf<C>
where
    C: AsComponent,
{
    pub const ROOT: Self = Self { components: Vec::new() };

    pub fn parse<'input>(input: &'input str) -> Result<Self, InvalidPath>
    where
        C: TryFrom<&'input str, Error = InvalidComponent> + 'input,
    {
        let mut this = Self::ROOT;

        if !input.is_empty() {
            for component_str in input.split('/') {
                this.components.push(C::try_from(component_str)?);
            }
        }

        Ok(this)
    }
}

impl<'input, C> TryFrom<&'input str> for PathBuf<C>
where
    C: AsComponent + TryFrom<&'input str, Error = InvalidComponent> + 'input,
{
    type Error = InvalidPath;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

#[derive(Debug)]
struct ArrayBuilder<C, const N: usize>
where
    C: AsComponent,
{
    valid_len: usize,
    components: [MaybeUninit<C>; N],
}

impl<C, const N: usize> ArrayBuilder<C, N>
where
    C: AsComponent,
{
    fn new() -> Self {
        unsafe {
            let components = MaybeUninit::uninit().assume_init();
            Self { valid_len: 0, components }
        }
    }

    fn push(&mut self, component: C) -> Result<(), InvalidPath> {
        let index = self.valid_len;
        match self.components.get_mut(index) {
            Some(element) => {
                unsafe {
                    element.as_mut_ptr().write(component);
                }
                self.valid_len += 1;
                Ok(())
            },
            None => Err(InvalidPath::PathToBig { buf_len: N }),
        }
    }

    fn finish(mut self) -> Result<ArrayPath<C, N>, InvalidPath> {
        if self.valid_len == N {
            self.valid_len = 0;
            unsafe {
                let components_raw = mem::replace(
                    &mut self.components,
                    MaybeUninit::uninit().assume_init(),
                );
                let components = components_raw.map(|comp| comp.assume_init());
                Ok(ArrayPath { components })
            }
        } else {
            Err(InvalidPath::PathToSmall { buf_len: N })
        }
    }
}

impl<C, const N: usize> Drop for ArrayBuilder<C, N>
where
    C: AsComponent,
{
    fn drop(&mut self) {
        for element in &mut self.components[..] {
            unsafe {
                element.as_mut_ptr().drop_in_place();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ArrayPath<C, const N: usize>
where
    C: AsComponent,
{
    pub components: [C; N],
}

impl<C> ArrayPath<C, 0>
where
    C: AsComponent,
{
    pub const ROOT: Self = Self { components: [] };
}

impl<C, const N: usize> ArrayPath<C, N>
where
    C: AsComponent,
{
    pub fn parse<'input>(input: &'input str) -> Result<Self, InvalidPath>
    where
        C: TryFrom<&'input str, Error = InvalidComponent> + 'input,
    {
        let mut builder = ArrayBuilder::new();

        if !input.is_empty() {
            for compomnent_str in input.split('/') {
                builder.push(C::try_from(compomnent_str)?)?;
            }
        }

        builder.finish()
    }
}

impl<'input, C, const N: usize> TryFrom<&'input str> for ArrayPath<C, N>
where
    C: AsComponent + TryFrom<&'input str, Error = InvalidComponent> + 'input,
{
    type Error = InvalidPath;

    fn try_from(input: &'input str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Path<C>
where
    C: AsComponent,
{
    pub components: [C],
}

impl<C> Path<C>
where
    C: AsComponent + 'static,
{
    pub const ROOT: &Self = Self::new(&[]);
}

impl<C> Path<C>
where
    C: AsComponent,
{
    pub const fn new(components: &[C]) -> &Self {
        unsafe { mem::transmute(components) }
    }

    pub fn new_mut(components: &mut [C]) -> &mut Self {
        unsafe { mem::transmute(components) }
    }

    pub const fn new_boxed(components: Box<[C]>) -> Box<Self> {
        unsafe { mem::transmute(components) }
    }
}

pub trait AsPath {
    type Component: AsComponent;

    fn as_path(&self) -> &Path<Self::Component>;
}

impl<C> AsPath for PathBuf<C>
where
    C: AsComponent,
{
    type Component = C;

    fn as_path(&self) -> &Path<Self::Component> {
        Path::new(&self.components[..])
    }
}

impl<C, const N: usize> AsPath for ArrayPath<C, N>
where
    C: AsComponent,
{
    type Component = C;

    fn as_path(&self) -> &Path<Self::Component> {
        Path::new(&self.components[..])
    }
}

impl<C, const N: usize> AsPath for [C; N]
where
    C: AsComponent,
{
    type Component = C;

    fn as_path(&self) -> &Path<Self::Component> {
        Path::new(&self[..])
    }
}

impl<'path, C> AsPath for &'path [C]
where
    C: AsComponent,
{
    type Component = C;

    fn as_path(&self) -> &Path<Self::Component> {
        Path::new(&self[..])
    }
}

impl<C> AsPath for Path<C>
where
    C: AsComponent,
{
    type Component = C;

    fn as_path(&self) -> &Path<Self::Component> {
        self
    }
}

impl<'this, P> AsPath for &'this P
where
    P: AsPath + ?Sized,
{
    type Component = P::Component;

    fn as_path(&self) -> &Path<Self::Component> {
        (**self).as_path()
    }
}

impl<P> AsPath for Box<P>
where
    P: AsPath + ?Sized,
{
    type Component = P::Component;

    fn as_path(&self) -> &Path<Self::Component> {
        (**self).as_path()
    }
}

impl<P> AsPath for Rc<P>
where
    P: AsPath + ?Sized,
{
    type Component = P::Component;

    fn as_path(&self) -> &Path<Self::Component> {
        (**self).as_path()
    }
}

impl<P> AsPath for Arc<P>
where
    P: AsPath + ?Sized,
{
    type Component = P::Component;

    fn as_path(&self) -> &Path<Self::Component> {
        (**self).as_path()
    }
}
