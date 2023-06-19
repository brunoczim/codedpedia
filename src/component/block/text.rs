//! This module exports block text components.

use super::BlockComponent;
use crate::{
    component::{Component, InlineComponent},
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

/// This components wraps another component and its text bold.
///
/// # HTML Classes
///
/// - `pedia-bold` attached to a `<div>` element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bold<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Bold<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for Bold<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<div class=\"pedia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</div>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Bold<C>
where
    C: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<b>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</b>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Bold<C>
where
    C: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// This components wraps another component and its text italic.
///
/// # HTML Classes
///
/// - `pedia-italic` attached to a `<div>` element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Italic<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Italic<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for Italic<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<div class=\"pedia-italic\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</div>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Italic<C>
where
    C: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<i>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</i>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Italic<C>
where
    C: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// This components wraps another component and its text preformatted. Suitable
/// for code.
///
/// # HTML Classes
///
/// - `pedia-preformatted` attached to a `<div>` element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = BlockComponent>;

impl<C> Component for Preformatted<C>
where
    C: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for Preformatted<C>
where
    C: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"pedia-preformatted\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</div>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Preformatted<C>
where
    C: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<pre>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</pre>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Preformatted<C>
where
    C: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// Component that takes a portion of inline components and puts it into a
/// paragraph.
///
/// # HTML Classes
///
/// - `pedia-paragraph` attached to a `<p>` element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Paragraph<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Paragraph<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<C> Render<Html> for Paragraph<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<p class=\"pedia-paragraph\">")?;
        self.0.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("<p>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Paragraph<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        Ok(())
    }
}

impl<C> Render<Text> for Paragraph<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Bold, Italic, Paragraph, Preformatted};
    use crate::{
        component::{block::InlineBlock, BlockComponent},
        location::InternalPath,
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn bold_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Bold(InlineBlock("abc")),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn italic_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Italic(InlineBlock("abc")),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn preformatted_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Preformatted(InlineBlock("abc")),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn paragraph_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Paragraph("abc"),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
