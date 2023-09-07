use super::{Format, Render};
use crate::component::Component;
use std::path::Component;

pub trait RenderExt<W>: Render<W> + Sized
where
    W: Format + ?Sized,
{
    fn then<R>(self, next: R) -> Then<Self, R>
    where
        R: Render<W>;
}

#[derive(Debug, Clone)]
pub struct Then<R, S> {
    first: R,
    second: S,
}

impl<R, S> Component for Then<R, S>
where
    R: Component,
    S: Component<Kind = R::Kind>,
{
    type Kind = R::Kind;
}
