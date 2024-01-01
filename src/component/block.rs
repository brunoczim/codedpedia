//! This module exports components that are of type block, as well their kind.

use std::fmt::{self, Write};

use crate::{
    domain::{
        component::{self, Component},
        render,
        Render,
        Renderer,
    },
    format::{Html, Markdown, Text},
};

use super::InlineComponent;

pub mod text;
pub mod media;
pub mod list;
/*
pub mod table;
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockComponent;

impl component::Kind for BlockComponent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InlineBlock<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for InlineBlock<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for InlineBlock<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<span class=\"pedia-inline-block\">")?;
        self.0.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for InlineBlock<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx.with_kind(&InlineComponent))
    }
}

impl<C> Render<Text> for InlineBlock<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx.with_kind(&InlineComponent))
    }
}
