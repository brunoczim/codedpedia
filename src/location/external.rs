use std::{rc::Rc, sync::Arc};

pub use url::Url as External;

pub trait AsExternal {
    fn as_external(&self) -> &External;
}

impl AsExternal for External {
    fn as_external(&self) -> &External {
        self
    }
}

impl<'this, E> AsExternal for &'this E
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Box<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Rc<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}

impl<E> AsExternal for Arc<E>
where
    E: AsExternal + ?Sized,
{
    fn as_external(&self) -> &External {
        (**self).as_external()
    }
}
