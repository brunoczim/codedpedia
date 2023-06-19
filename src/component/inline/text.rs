//! This module exports inline text components.

use super::InlineComponent;
use crate::{
    component::Component,
    location::Location,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

/// This components wraps another component and its text bold.
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
        ctx: Context<Self::Kind>,
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
        ctx: Context<Self::Kind>,
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
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// This components wraps another component and its text italic.
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
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        renderer.write_str("<span class=\"pedia-bold\">")?;
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
        ctx: Context<Self::Kind>,
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
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// This components wraps another component and its text preformatted. Suitable
/// for code.
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
        ctx: Context<Self::Kind>,
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
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}

/// This component is embeds a link to another resource in a component.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Link<C>
where
    C: Component<Kind = InlineComponent>,
{
    /// Target displayed component.
    pub target: C,
    /// The embedded link.
    pub location: Location,
}

impl<C> Component for Link<C>
where
    C: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;
}

impl<C> Render<Html> for Link<C>
where
    C: Render<Html, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<a class=\"pedia-link\" href=\"")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str("\">")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("</a>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Link<C>
where
    C: Render<Markdown, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.target.render(renderer, ctx)?;
        renderer.write_str("](")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str(")")?;
        Ok(())
    }
}

impl<C> Render<Text> for Link<C>
where
    C: Render<Text, Kind = InlineComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.target.render(renderer, ctx)
    }
}

#[cfg(test)]
mod test {
    use super::{Bold, Italic, Link, Preformatted};
    use crate::{
        component::InlineComponent,
        location::{InternalPath, Location},
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
            Bold("abc"),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn italic_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Italic("abc"),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn preformatted_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Preformatted("abc"),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn link_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Link { location: Location::internal(""), target: "abc" },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
