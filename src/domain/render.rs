use std::{
    fmt,
    rc::Rc,
    sync::{Arc, Mutex},
};

use crate::location;

use super::{
    component::{Component, Kind},
    format::Format,
};

pub type Dyn<'obj, W, K> = Arc<dyn Render<W, Kind = K> + Send + Sync + 'obj>;

pub type DynUnsync<'obj, W, K> = Rc<dyn Render<W, Kind = K> + 'obj>;

pub trait Render<W>: Component
where
    W: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result;

    fn into_dyn<'obj>(self) -> Dyn<'obj, W, Self::Kind>
    where
        Self: Sized + Send + Sync + 'obj,
    {
        Arc::new(self)
    }

    fn into_dyn_unsync<'obj>(self) -> DynUnsync<'obj, W, Self::Kind>
    where
        Self: Sized + 'obj,
    {
        Rc::new(self)
    }
}

impl<'this, T, W> Render<W> for &'this T
where
    W: Format + ?Sized,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<'this, T, W> Render<W> for &'this mut T
where
    W: Format + ?Sized,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Box<T>
where
    W: Format + ?Sized,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Rc<T>
where
    W: Format + ?Sized,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, W> Render<W> for Arc<T>
where
    W: Format + ?Sized,
    T: Render<W> + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl<T, R, const N: usize> Render<R> for [T; N]
where
    T: Render<R>,
    R: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for element in self {
            element.render(renderer, ctx)?;
        }
        Ok(())
    }
}

impl<T, R> Render<R> for [T]
where
    T: Render<R>,
    R: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for element in self {
            element.render(renderer, ctx)?;
        }
        Ok(())
    }
}

impl<T, R> Render<R> for Vec<T>
where
    T: Render<R>,
    R: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for element in self {
            element.render(renderer, ctx)?;
        }
        Ok(())
    }
}

impl<A, B, R> Render<R> for (A, B)
where
    A: Render<R>,
    B: Render<R, Kind = A::Kind>,
    R: Format + ?Sized,
{
    fn render(
        &self,
        renderer: &mut Renderer<R>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)?;
        self.1.render(renderer, ctx)?;
        Ok(())
    }
}

pub trait Scope {
    type Format: Format + ?Sized;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T;
}

impl<'this, S> Scope for &'this S
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<'this, S> Scope for &'this mut S
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Box<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Rc<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

impl<S> Scope for Arc<S>
where
    S: Scope + ?Sized,
{
    type Format = S::Format;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        (**self).enter(format, consumer)
    }
}

pub struct Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    format: &'format mut W,
    target: &'target mut (dyn fmt::Write + 'obj),
}

impl<'format, 'target, 'obj, W> fmt::Debug
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized + fmt::Debug,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Renderer")
            .field("format", &self.format)
            .field("formatter", &(self.target as *const _))
            .finish()
    }
}

impl<'format, 'target, 'obj, W> Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    pub fn new(
        format: &'format mut W,
        target: &'target mut (dyn fmt::Write + 'obj),
    ) -> Self {
        Self { format, target }
    }

    pub fn scoped<S, F, T>(&mut self, scope: S, consumer: F) -> T
    where
        F: FnOnce(&mut Renderer<W>) -> T,
        S: Scope<Format = W>,
    {
        let render_format = &mut *self.format;
        let formatter = &mut *self.target;

        scope.enter(render_format, |render_format| {
            consumer(&mut Renderer { format: render_format, target: formatter })
        })
    }
}

impl<'format, 'target, 'obj, W> fmt::Write
    for Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    fn write_str(&mut self, input: &str) -> fmt::Result {
        self.format.write_str(input, self.target)
    }
}

#[derive(Debug)]
pub struct Context<'loc, 'kind, K>
where
    K: Kind + ?Sized,
{
    location: &'loc location::Path,
    level: u32,
    kind: &'kind K,
}

impl<'loc, 'kind, K> Clone for Context<'loc, 'kind, K>
where
    K: Kind + ?Sized,
{
    fn clone(&self) -> Self {
        Self { location: self.location, level: self.level, kind: self.kind }
    }
}

impl<'loc, 'kind, K> Copy for Context<'loc, 'kind, K> where K: Kind + ?Sized {}

impl<'loc, 'kind, K> Context<'loc, 'kind, K>
where
    K: Kind + ?Sized,
{
    pub fn new(location: &'loc location::Path, kind: &'kind K) -> Self {
        Self { location, level: 0, kind }
    }

    pub fn with_kind<'k0, Q>(self, kind: &'k0 Q) -> Context<'loc, 'k0, Q>
    where
        Q: Kind + ?Sized,
    {
        Context { location: self.location, level: self.level, kind }
    }

    pub fn location(self) -> &'loc location::Path {
        self.location
    }

    pub fn section_level(self) -> u32 {
        self.level
    }

    pub fn enter_section(self) -> Self {
        Self { level: self.level + 1, ..self }
    }

    pub fn kind(self) -> &'kind K {
        self.kind
    }
}

#[derive(Debug)]
pub struct RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    component: C,
    format: Mutex<&'format mut W>,
    context: Context<'loc, 'kind, C::Kind>,
}

impl<'loc, 'kind, 'format, C, W> RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    pub fn new(
        component: C,
        format: &'format mut W,
        context: Context<'loc, 'kind, C::Kind>,
    ) -> Self {
        Self { component, format: Mutex::new(format), context }
    }
}

impl<'loc, 'kind, 'format, C, W> fmt::Display
    for RenderAsDisplay<'loc, 'kind, 'format, C, W>
where
    C: Render<W>,
    W: Format + ?Sized,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut format = self.format.lock().unwrap();
        self.component
            .render(&mut Renderer::new(&mut **format, fmtr), self.context)?;
        Ok(())
    }
}
