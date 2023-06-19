//! This module defines media that can be inlined in the text.

use super::InlineComponent;
use crate::{
    component::Component,
    location::Location,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::fmt::{self, Write};

/// A playable audio.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Audio {
    /// Where the audio file is.
    pub location: Location,
    /// Alternative text description of the audio.
    pub alt: String,
}

impl Component for Audio {
    type Kind = InlineComponent;
}

impl Render<Html> for Audio {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<audio class=\"pedia-audio\" controls src=\"")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str("\">")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("</audio>")?;
        Ok(())
    }
}

impl Render<Markdown> for Audio {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<audio class=\"pedia-audio\" controls src=\"")?;
        self.location.render(renderer, ctx)?;
        renderer.write_str("\">")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("</audio>")?;
        Ok(())
    }
}

impl Render<Text> for Audio {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("[")?;
        self.alt.render(renderer, ctx)?;
        renderer.write_str("]")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Audio;
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
    fn audio_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Audio {
                location: Location::internal("abc/def.ogg"),
                alt: String::from("audio about def"),
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
