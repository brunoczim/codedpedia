use super::BlockComponent;
use crate::{
    component::InlineComponent,
    domain::{component::Component, render, Render, Renderer},
    format::{Html, Markdown, Text},
};
use std::fmt::{self, Write};

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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.0.render(renderer, ctx)
    }
}

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
        ctx: render::Context<Self::Kind>,
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
    C: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        self.0.render(renderer, ctx)
    }
}

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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
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
        ctx: render::Context<Self::Kind>,
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
        component::block::{BlockComponent, InlineBlock},
        domain::render::{self, RenderAsDisplay},
        format::{html::test::validate_html_fragment, Html},
        location,
    };

    #[test]
    fn bold_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Bold(InlineBlock("abc")),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn italic_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Italic(InlineBlock("abc")),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn preformatted_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Preformatted(InlineBlock("abc")),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn paragraph_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Paragraph("abc"),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
