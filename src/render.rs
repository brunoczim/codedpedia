//! This module exports items related to rendering components.

use crate::{
    component::{Component, ComponentKind},
    hseq::coproduct::{Cocons, Conil},
    location::InternalPath,
};
pub use html::Html;
pub use markdown::Markdown;
use std::{
    fmt,
    rc::Rc,
    sync::{Arc, Mutex},
};
pub use text::Text;

pub mod html;
pub mod markdown;
pub mod text;

mod common_text;

/// A rendering format: e.g. HTML, Markdown, etc.
pub trait Format {
    /// A proxy method for writes to a target formatter that is used when
    /// rendering.
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result;
}

impl<'this, W> Format for &'this mut W
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

impl<W> Format for Box<W>
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

/// A scope of a render format. The scope allows a component to change rendering
/// configuration by entering the scope.
pub trait Scope {
    /// Rendering format associated with this scope.
    type Format: Format + ?Sized;

    /// Enters this scope during the call of the given consumer function; exits
    /// when this method terminates.
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

/// General rendering trait. A component implements this trait when it can
/// renders using the given format `W`.
pub trait Render<W>: Component
where
    W: Format + ?Sized,
{
    /// Renders the given component using two things:
    /// - A renderer backed by a render format;
    /// - A rendering context, providing things such page location or section
    ///   level.
    ///
    /// [`Renderer`] implements [`std::fmt::Write`], so arbitrary strings can be
    /// written to it, HOWEVER, it should be reserved only for writing syntax to
    /// the renderer, strings as plain data should be written using their
    /// [`Render::render`] method.
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result;
}

impl<C, W> Render<W> for Conil<C>
where
    C: ComponentKind,
    W: Format + ?Sized,
{
    fn render(
        &self,
        _renderer: &mut Renderer<W>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        Ok(())
    }
}

impl<W, H, T> Render<W> for Cocons<H, T>
where
    W: Format + ?Sized,
    H: Render<W>,
    T: Render<W, Kind = H::Kind>,
{
    fn render(
        &self,
        renderer: &mut Renderer<W>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        match self {
            Cocons::Head(head) => head.render(renderer, ctx),
            Cocons::Tail(tail) => tail.render(renderer, ctx),
        }
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

/// An auto-trait for components that implement rendering for all supported
/// formats.
pub trait FullRender: Render<Html> + Render<Markdown> + Render<Text> {
    /// Converts the fully renderable component into a trait object wrapped by a
    /// shared reference. This method is intended for thread-safe components. If
    /// your component is not thread-safe or does not care about it, use
    /// [`FullRender::into_dyn_unsync`] instead.
    ///
    /// This method is intended to improve ergonomics of programmers who opt for
    /// dynamic dispatch.
    fn into_dyn<'obj>(self) -> DynFullComponent<'obj, Self::Kind>
    where
        Self: Sized + Send + Sync + 'obj,
    {
        Arc::new(self)
    }

    /// Converts the fully renderable component into a trait object wrapped by a
    /// shared reference. This method is intended for non-thread-safe
    /// components, or for components that do not care about thread-safety.
    /// If your component is thread-safe and cares about it, use
    /// [`FullRender::into_dyn`] instead.
    ///
    /// This method is intended to improve ergonomics of programmers who opt for
    /// dynamic dispatch.
    fn into_dyn_unsync<'obj>(self) -> DynFullComponentUnsync<'obj, Self::Kind>
    where
        Self: Sized + 'obj,
    {
        Rc::new(self)
    }
}

/// A dynamic trait object for thread-safe components with full render support.
pub type DynFullComponent<'obj, K> =
    Arc<dyn FullRender<Kind = K> + Send + Sync + 'obj>;

/// A dynamic trait object for thread-unsafe components with full render
/// support.
pub type DynFullComponentUnsync<'obj, K> = Rc<dyn FullRender<Kind = K> + 'obj>;

impl<T> FullRender for T where
    T: Render<Html> + Render<Markdown> + Render<Text> + ?Sized
{
}

/// A proxy formatter for rendering using Rust's formatting and a render format.
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
            .field("render_format", &self.format)
            .field("formatter", &(self.target as *const _))
            .finish()
    }
}

impl<'format, 'target, 'obj, W> Renderer<'format, 'target, 'obj, W>
where
    W: Format + ?Sized,
{
    /// Creates a new renderer given a render format and a target formatter.
    pub fn new(
        format: &'format mut W,
        target: &'target mut (dyn fmt::Write + 'obj),
    ) -> Self {
        Self { format, target }
    }

    /// Given a scope over the render format and a scope consumer, enters the
    /// given scope.
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

/// Context data of a session of rendering.
#[derive(Debug)]
pub struct Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    location: &'loc InternalPath,
    level: u32,
    kind: &'kind K,
}

impl<'loc, 'kind, K> Clone for Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    fn clone(&self) -> Self {
        Self { location: self.location, level: self.level, kind: self.kind }
    }
}

impl<'loc, 'kind, K> Copy for Context<'loc, 'kind, K> where
    K: ComponentKind + ?Sized
{
}

impl<'loc, 'kind, K> Context<'loc, 'kind, K>
where
    K: ComponentKind + ?Sized,
{
    /// Creates a context from page location and component kind.
    pub fn new(location: &'loc InternalPath, kind: &'kind K) -> Self {
        Self { location, level: 0, kind }
    }

    /// Recreates the context but with another  component kind.
    pub fn with_kind<Q>(self, kind: &'kind Q) -> Context<'loc, 'kind, Q>
    where
        Q: ComponentKind + ?Sized,
    {
        Context { location: self.location, level: self.level, kind }
    }

    /// Yields the location of the page being rendered.
    pub fn location(self) -> &'loc InternalPath {
        self.location
    }

    /// Yields the current nesting level of sections, starting from 0.
    pub fn section_level(self) -> u32 {
        self.level
    }

    /// Enters another section level, by incrementing it.
    pub fn enter_section(self) -> Self {
        Self { level: self.level + 1, ..self }
    }

    /// Yields the component kind.
    pub fn kind(self) -> &'kind K {
        self.kind
    }
}

/// A helper type that will render a component using Rust's [`fmt::Display`]
/// trait.
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
    /// Creates a new display rendering helper given a component, a render
    /// format and an initial context.
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
