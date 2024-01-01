//! This module defines media that can be inlined in the text.

use crate::{
    component::location::LocationComponent,
    domain::{
        component::Component,
        render::{self, Render, Renderer},
    },
    format::{Html, Markdown, Text},
};

use super::InlineComponent;
use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Audio<L>
where
    L: Component<Kind = LocationComponent>,
{
    pub location: L,
    pub alt: String,
}

impl<L> Component for Audio<L>
where
    L: Component<Kind = LocationComponent>,
{
    type Kind = InlineComponent;
}

impl<L> Render<Html> for Audio<L>
where
    L: Render<Html, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<audio class=\"pedia-audio\" controls src=\"")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str("\">")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("</audio>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Audio<L>
where
    L: Render<Markdown, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<audio class=\"pedia-audio\" controls src=\"")?;
        self.location.render(renderer, ctx.with_kind(&LocationComponent))?;
        renderer.write_str("\">")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("</audio>")?;
        Ok(())
    }
}

impl<L> Render<Text> for Audio<L>
where
    L: Render<Text, Kind = LocationComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("]")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        component::InlineComponent,
        domain::render::{self, RenderAsDisplay},
        format::{html::test::validate_html_fragment, Html},
        location::{self, Location},
    };

    use super::Audio;

    #[test]
    fn audio_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Audio {
                location: Location::new("abc/def.ogg").unwrap(),
                alt: String::from("audio about def"),
            },
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
