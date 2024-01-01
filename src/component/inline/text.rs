//! This module exports inline text components.

use super::InlineComponent;
use crate::{
    component::location::LocationComponent,
    domain::{render, Component, Render, Renderer},
    format::{Html, Markdown, Text},
};
use std::fmt::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bold<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Bold<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Bold<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"pedia-bold\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Bold<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("**")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("**")?;
        Ok(())
    }
}

impl<C> Render<Text> for Bold<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Italic<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Italic<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Italic<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"pedia-italic\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Italic<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("_")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("_")?;
        Ok(())
    }
}

impl<C> Render<Text> for Italic<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Preformatted<C>(pub C)
where
    C: Component<Kind = InlineComponent>;

impl<C> Component for Preformatted<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Preformatted<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<span class=\"pedia-preformatted\">")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</span>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Preformatted<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<pre>")?;
        self.0.render(renderer, ctx)?;
        renderer.write_str("</pre>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Preformatted<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Link<C, L>
where
    C: Component<Kind = InlineComponent>,
    L: Component<Kind = LocationComponent>,
{
    pub target: C,
    pub location: L,
}

impl<C, L> Component for Link<C, L>
where
    C: Component<Kind = InlineComponent>,
    L: Component<Kind = LocationComponent>,
{
    type Kind = InlineComponent;
}

impl<C, L> Render<Html> for Link<C, L>
where
    C: Render<Html, Kind = InlineComponent>,
    L: Render<Html, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<a class=\"pedia-link\" href=\"")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str("\">")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("</a>")?;
        Ok(())
    }
}

impl<C, L> Render<Markdown> for Link<C, L>
where
    C: Render<Markdown, Kind = InlineComponent>,
    L: Render<Markdown, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str(")")?;
        Ok(())
    }
}

impl<C, L> Render<Text> for Link<C, L>
where
    C: Render<Text, Kind = InlineComponent>,
    L: Render<Text, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        self.target.render(renderer, ctx)
    }
}

#[cfg(test)]
mod test {
    use super::{Bold, Italic, Link, Preformatted};
    use crate::{
        component::InlineComponent,
        domain::{render, RenderAsDisplay},
        format::{html::test::validate_html_fragment, Html},
        location::{self, Location},
    };

    #[test]
    fn bold_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Bold("abc"),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn italic_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Italic("abc"),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn preformatted_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Preformatted("abc"),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn link_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Link { location: <&Location>::default(), target: "abc" },
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
