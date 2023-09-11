use std::{mem, rc::Rc, sync::Arc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Component {
    contents: str,
}

impl Component {
    pub fn parse(input: &str) -> Result<&Self, InvalidId> {
        Self::check_parse_input(input)?;
        Ok(Self::from_ref(input))
    }

    pub fn parse_owned(input: Box<str>) -> Result<Box<Self>, InvalidId> {
        Self::check_parse_input(input.as_ref())?;
        Ok(Self::from_box(input))
    }

    fn from_ref(input: &str) -> &Self {
        unsafe { mem::transmute(input) }
    }

    fn from_box(input: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(input) }
    }

    fn check_parse_input(input: &str) -> Result<(), InvalidId> {
        todo!()
    }
}

pub trait AsComponent {
    fn as_component(&self) -> &Component;
}

impl AsComponent for Component {
    fn as_component(&self) -> &Component {
        self
    }
}

impl<'this, C> AsComponent for &'this C
where
    C: AsComponent + ?Sized,
{
    fn as_component(&self) -> &Component {
        (**self).as_component()
    }
}

impl<C> AsComponent for Box<C>
where
    C: AsComponent + ?Sized,
{
    fn as_component(&self) -> &Component {
        (**self).as_component()
    }
}

impl<C> AsComponent for Rc<C>
where
    C: AsComponent + ?Sized,
{
    fn as_component(&self) -> &Component {
        (**self).as_component()
    }
}

impl<C> AsComponent for Arc<C>
where
    C: AsComponent + ?Sized,
{
    fn as_component(&self) -> &Component {
        (**self).as_component()
    }
}
