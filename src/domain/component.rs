use std::{fmt, rc::Rc, sync::Arc};

pub trait Kind {}

impl<'this, K> Kind for &'this K where K: Kind + ?Sized {}

impl<'this, K> Kind for &'this mut K where K: Kind + ?Sized {}

impl<K> Kind for Box<K> where K: Kind + ?Sized {}

impl<K> Kind for Rc<K> where K: Kind + ?Sized {}

impl<K> Kind for Arc<K> where K: Kind + ?Sized {}

pub trait Component: fmt::Debug {
    type Kind: Kind + ?Sized;
}

impl<'this, T> Component for &'this T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<'this, T> Component for &'this mut T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Box<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Rc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T> Component for Arc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;
}

impl<T, const N: usize> Component for [T; N]
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<T> Component for [T]
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<T> Component for Vec<T>
where
    T: Component,
{
    type Kind = T::Kind;
}

impl<A, B> Component for (A, B)
where
    A: Component,
    B: Component<Kind = A::Kind>,
{
    type Kind = A::Kind;
}
