//! This module exports the component trait and components provided by this
//! crate.

pub mod inline;
pub mod block;
pub mod section;
pub mod asset;
pub mod page;

use crate::hseq::coproduct::{Cocons, Conil};
pub use block::BlockComponent;
pub use inline::InlineComponent;
use std::{fmt, rc::Rc, sync::Arc};

/// A component kind is a way of "typing" (i.e. give types) to components at a
/// trait level.
pub trait ComponentKind {}

/// A component's base trait. A component is a piece of data or UI logic that
/// will be rendered into UI.
pub trait Component: fmt::Debug {
    /// The kind, i.e. the "type", of this component.
    type Kind: ComponentKind + ?Sized;
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

impl<C> Component for Conil<C>
where
    C: ComponentKind,
{
    type Kind = C;
}

impl<H, T> Component for Cocons<H, T>
where
    H: Component,
    T: Component<Kind = H::Kind>,
{
    type Kind = H::Kind;
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

impl<'this, K> ComponentKind for &'this K where K: ComponentKind + ?Sized {}

impl<'this, K> ComponentKind for &'this mut K where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Box<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Rc<K> where K: ComponentKind + ?Sized {}

impl<K> ComponentKind for Arc<K> where K: ComponentKind + ?Sized {}
