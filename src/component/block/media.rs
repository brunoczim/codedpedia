//! This module exports block media components.

use super::BlockComponent;
use crate::{
    component::{location::LocationComponent, InlineComponent},
    domain::{component::Component, render, Render, Renderer},
    format::{Html, Markdown, Text},
};
use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image<L>
where
    L: Component<Kind = LocationComponent>,
{
    pub location: L,
    pub alt: String,
}

impl<L> Component for Image<L>
where
    L: Component<Kind = LocationComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for Image<L>
where
    L: Render<Html, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<img class=\"pedia-image\" src=\"")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str("\" alt=\"")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\">")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Image<L>
where
    L: Render<Markdown, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str(")\n\n")?;
        Ok(())
    }
}

impl<L> Render<Text> for Image<L>
where
    L: Render<Text, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("]")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Figure<L, T>
where
    L: Component<Kind = LocationComponent>,
    T: Component<Kind = InlineComponent>,
{
    /// The targetted image.
    pub image: Image<L>,
    /// Legend of the image.
    pub legend: T,
}

impl<L, T> Component for Figure<L, T>
where
    L: Component<Kind = LocationComponent>,
    T: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;
}

impl<L, T> Render<Html> for Figure<L, T>
where
    L: Render<Html, Kind = LocationComponent>,
    T: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"pedia-figure\">")?;
        self.image.render(renderer, ctx)?;
        renderer.write_str("<div class=\"pedia-figure-legend\">")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</div></div>")?;
        Ok(())
    }
}

impl<L, T> Render<Markdown> for Figure<L, T>
where
    L: Render<Markdown, Kind = LocationComponent>,
    T: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("![")?;
        self.image.alt.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("](")?;
        self.image
            .location
            .render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str(")\n")?;
        self.legend.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        Ok(())
    }
}

impl<L, T> Render<Text> for Figure<L, T>
where
    L: Render<Text, Kind = LocationComponent>,
    T: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
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
    use crate::{
        component::{block::BlockComponent, inline::text::Bold},
        domain::render::{self, RenderAsDisplay},
        format::{html::test::validate_html_fragment, Html},
        location,
    };

    use super::{Figure, Image};

    #[test]
    fn image_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Image {
                location: location::Path::new("abc/hi.png").unwrap(),
                alt: String::from("img about hi"),
            },
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn figure_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Figure {
                image: Image {
                    location: location::Path::new("haha/scream.png").unwrap(),
                    alt: String::from("image"),
                },
                legend: Bold("stark image"),
            },
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
