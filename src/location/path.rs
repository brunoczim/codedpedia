use std::{mem, rc::Rc, sync::Arc};

use super::component::AsComponent;

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
    C: AsComponent,
{
    fn from_ref(components: &[C]) -> &Self {
        unsafe { mem::transmute(components) }
    }

    fn from_box(components: Box<[C]>) -> Box<Self> {
        unsafe { mem::transmute(components) }
    }
}

pub trait AsPath {
    type Component: AsComponent;

    fn as_path(&self) -> &Path<Self::Component>;
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
