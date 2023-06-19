//! This module exports block media components.

use super::BlockComponent;
use crate::{
    component::{Component, InlineComponent},
    location::Location,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

/// An image component.
///
/// # HTML Classes
///
/// - `pedia-image` attached to an `<img>` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image {
    /// Where the image is.
    pub location: Location,
    /// Alternative text describing the image.
    pub alt: String,
}

impl Component for Image {
    type Kind = BlockComponent;
}

impl Render<Html> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<img class=\"pedia-image\" src=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\" alt=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\">")?;
        Ok(())
    }
}

impl Render<Markdown> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")\n\n")?;
        Ok(())
    }
}

impl Render<Text> for Image {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("]")?;
        Ok(())
    }
}

/// A figure component, an image with legend.
///
/// # HTML Classes
///
/// - `pedia-figure` attached to a `<div>` element.
/// - `pedia-figure-legend` attached to a `<div>` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    /// The targetted image.
    pub image: Image,
    /// Legend of the image.
    pub legend: L,
}

impl<L> Component for Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for Figure<L>
where
    L: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"pedia-figure\">")?;
        self.image.render(renderer, ctx)?;
        renderer.write_str("<div class=\"pedia-figure-legend\">")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</div></div>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Figure<L>
where
    L: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.image.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.image
            .location
            .render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")\n")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        Ok(())
    }
}

impl<L> Render<Text> for Figure<L>
where
    L: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.image.render(renderer, ctx)?;
        renderer.write_str("(")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str(")")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Figure, Image};
    use crate::{
        component::{inline::text::Bold, BlockComponent},
        location::{InternalPath, Location},
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn image_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Image {
                location: Location::internal("abc/hi.png"),
                alt: String::from("img about hi"),
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn figure_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Figure {
                image: Image {
                    location: Location::internal("haha/scream.png"),
                    alt: String::from("imaaage"),
                },
                legend: Bold("stark image"),
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
