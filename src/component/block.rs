//! This module exports components that are of type block, as well their kind.

use super::{Component, ComponentKind, InlineComponent};
use crate::render::{Context, Html, Markdown, Render, Renderer, Text};
use std::fmt::{self, Write};

pub mod text;
pub mod media;
pub mod list;
pub mod table;

/// A block component. Such component is one that cannot appear in the middle of
/// reading text and can appear directly in the body of a section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockComponent;

impl ComponentKind for BlockComponent {}

/// Wrapper over inline components to make them a block component. Note,
/// however, that if you just use this in a section body, the outcome might not
/// be desirable, but if you use this in a table cell, for example, it makes
/// perfect sense.
///
/// # HTML Classes
///
/// - `pedia-inline-block` attached to a `<span>` element.
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
        ctx: Context<Self::Kind>,
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
        ctx: Context<Self::Kind>,
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
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx.with_kind(&InlineComponent))
    }
}
