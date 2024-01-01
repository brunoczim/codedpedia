use std::fmt::{self, Write};

use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

use crate::{
    domain::{
        component::{self, Component},
        render::{self, Renderer},
        Format,
        Render,
    },
    format::{Html, Markdown, Text},
    location::{self, Location},
};

const ENCODE_CHARS: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'<')
    .add(b'>')
    .add(b'\'')
    .add(b'"')
    .add(b'`')
    .add(b'\\')
    .add(b'[')
    .add(b']')
    .add(b'(')
    .add(b')')
    .add(b'&');

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LocationComponent;

impl component::Kind for LocationComponent {}

fn render_location_as_url<W>(
    location: &Location,
    renderer: &mut Renderer<W>,
    ctx: render::Context<LocationComponent>,
) -> fmt::Result
where
    W: Format,
{
    match location.view() {
        location::general::ViewRef::Internal(internal) => {
            render_internal_as_url(internal, renderer, ctx)
        },
        location::general::ViewRef::External(external) => {
            render_external_as_url(external, renderer, ctx)
        },
    }
}

fn render_internal_as_url<W>(
    internal: &location::Internal,
    renderer: &mut Renderer<W>,
    ctx: render::Context<LocationComponent>,
) -> fmt::Result
where
    W: Format,
{
    match internal.view() {
        location::internal::View::Path(path) => {
            render_path_as_url(path, renderer, ctx)?;
        },
        location::internal::View::Id(id) => {
            render_id_as_url(id, renderer, ctx)?;
        },
        location::internal::View::PathWithId(path, id) => {
            render_path_as_url(path, renderer, ctx)?;
            render_id_as_url(id, renderer, ctx)?;
        },
    }

    Ok(())
}

fn render_external_as_url<W>(
    external: &location::External,
    renderer: &mut Renderer<W>,
    _ctx: render::Context<LocationComponent>,
) -> fmt::Result
where
    W: Format,
{
    match external.view() {
        location::external::ViewRef::WithHost { .. } => {
            for piece in
                percent_encode(external.raw_contents().as_bytes(), ENCODE_CHARS)
            {
                renderer.write_str(piece)?;
            }
        },
        location::external::ViewRef::Other(rest) => {
            for piece in percent_encode(rest.as_bytes(), ENCODE_CHARS) {
                renderer.write_str(piece)?;
            }
        },
    }

    Ok(())
}

fn render_path_as_url<W>(
    path: &location::Path,
    renderer: &mut Renderer<W>,
    ctx: render::Context<LocationComponent>,
) -> fmt::Result
where
    W: Format,
{
    let mut this_components = path.components().peekable();
    let mut ctx_components = ctx.location().components().peekable();

    while this_components.peek().is_some()
        && this_components.peek() == ctx_components.peek()
    {
        this_components.next();
        ctx_components.next();
    }

    for _ in ctx_components {
        renderer.write_str("../")?;
    }

    for component in this_components {
        for piece in
            percent_encode(component.raw_contents().as_bytes(), ENCODE_CHARS)
        {
            renderer.write_str(piece)?;
        }
    }

    Ok(())
}

fn render_id_as_url<W>(
    id: &location::Id,
    renderer: &mut Renderer<W>,
    _ctx: render::Context<LocationComponent>,
) -> fmt::Result
where
    W: Format,
{
    renderer.write_str("#")?;
    for piece in percent_encode(id.raw_contents().as_bytes(), ENCODE_CHARS) {
        renderer.write_str(piece)?;
    }

    Ok(())
}

impl Component for Location {
    type Kind = LocationComponent;
}

impl Render<Html> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_location_as_url(self, renderer, ctx)
    }
}

impl Render<Markdown> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_location_as_url(self, renderer, ctx)
    }
}

impl Render<Text> for Location {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self.raw_contents())
    }
}

impl Component for location::Internal {
    type Kind = LocationComponent;
}

impl Render<Html> for location::Internal {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_internal_as_url(self, renderer, ctx)
    }
}

impl Render<Markdown> for location::Internal {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_internal_as_url(self, renderer, ctx)
    }
}

impl Render<Text> for location::Internal {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self.raw_contents())
    }
}

impl Component for location::External {
    type Kind = LocationComponent;
}

impl Render<Html> for location::External {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_external_as_url(self, renderer, ctx)
    }
}

impl Render<Markdown> for location::External {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_external_as_url(self, renderer, ctx)
    }
}

impl Render<Text> for location::External {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self.raw_contents())
    }
}

impl Component for location::Path {
    type Kind = LocationComponent;
}

impl Render<Html> for location::Path {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_path_as_url(self, renderer, ctx)
    }
}

impl Render<Markdown> for location::Path {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_path_as_url(self, renderer, ctx)
    }
}

impl Render<Text> for location::Path {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self.raw_contents())
    }
}

impl Component for location::Id {
    type Kind = LocationComponent;
}

impl Render<Html> for location::Id {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_id_as_url(self, renderer, ctx)
    }
}

impl Render<Markdown> for location::Id {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        render_id_as_url(self, renderer, ctx)
    }
}

impl Render<Text> for location::Id {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(".#")?;
        renderer.write_str(self.raw_contents())?;
        Ok(())
    }
}
