//! This module exports components that are of type inline, as well their kind.

use super::{Component, ComponentKind};
use crate::render::{Context, Html, Markdown, Render, Renderer, Text};
use std::fmt::{self, Write};

pub mod text;
pub mod media;

/// An inline component. Such component is one that can appear in the middle of
/// reading text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InlineComponent;

impl ComponentKind for InlineComponent {}

fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#39;"),
        '\\' => Some("&#92;"),
        '/' => Some("&#47;"),
        _ => None,
    }
}

fn md_escape(ch: char) -> Option<&'static str> {
    match ch {
        '*' => Some("\\*"),
        '-' => Some("\\-"),
        '`' => Some("\\`"),
        '_' => Some("\\_"),
        '(' => Some("\\("),
        ')' => Some("\\)"),
        '[' => Some("\\["),
        ']' => Some("\\]"),
        _ => html_escape(ch),
    }
}

impl Component for str {
    type Kind = InlineComponent;
}

impl Render<Html> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            renderer.write_str(&self[start .. end])?;
            renderer.write_str(escape)?;
            start = end + 1;
        }

        renderer.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<Markdown> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| md_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            renderer.write_str(&self[start .. end])?;
            renderer.write_str(escape)?;
            start = end + 1;
        }

        renderer.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Render<Text> for str {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        _ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str(self)
    }
}

impl Component for String {
    type Kind = InlineComponent;
}

impl Render<Html> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl Render<Markdown> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

impl Render<Text> for String {
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        (**self).render(renderer, ctx)
    }
}

#[cfg(test)]
mod test {
    use super::InlineComponent;
    use crate::{
        location::InternalPath,
        render::{Context, Html, Markdown, RenderAsDisplay, Text},
    };

    #[test]
    fn render_str_as_html_simple() {
        let rendered = RenderAsDisplay::new(
            "abc def g",
            &mut Html,
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        assert_eq!(rendered, "abc def g");
    }

    #[test]
    fn render_str_as_html_escape() {
        let rendered = RenderAsDisplay::new(
            "abc def \" g </> &",
            &mut Html,
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        assert_eq!(rendered, "abc def &quot; g &lt;&#47;&gt; &amp;");
    }

    #[test]
    fn render_str_as_md_simple() {
        let rendered = RenderAsDisplay::new(
            "abc def g",
            &mut Markdown::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        assert_eq!(rendered, "abc def g");
    }

    #[test]
    fn render_str_as_md_escape() {
        let rendered = RenderAsDisplay::new(
            "abc def \" g </> &",
            &mut Markdown::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        assert_eq!(rendered, "abc def &quot; g &lt;&#47;&gt; &amp;");
    }

    #[test]
    fn render_str_as_text() {
        let rendered = RenderAsDisplay::new(
            "abc def g",
            &mut Text::default(),
            Context::new(&InternalPath::default(), &InlineComponent),
        )
        .to_string();

        assert_eq!(rendered, "abc def g");
    }
}
