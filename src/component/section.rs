//! This module exports section components-related utilites.

use super::{BlockComponent, Component, ComponentKind, InlineComponent};
use crate::{
    hseq::IntoIterRef,
    location::{Id, InternalLoc, Location},
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::{
    cmp::Ordering,
    fmt::{self, Write},
    hash::{Hash, Hasher},
};

/// The kind of section-type components. A section component represent an
/// article's section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SectionComponent;

impl ComponentKind for SectionComponent {}

/// A page/article section.
///
/// # HTML Classes
///
/// - `pedia-section` and `pedia-section-N` in a `<div>` element depending on
///   the depth of the section (N will contain the depth starting from `0`).
/// - `pedia-title` in a `<h2>`, `<h3>`, etc element depending on the depth of
///   the section.
/// - `pedia-title-link` in an `<a>` element.
/// - `pedia-section-body-wrapper` in a `<div>` element, surrounding
///   `pedia-section-body` in a `<div>` element.
/// - `pedia-section-children` in a `<div>` element.
pub struct Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    /// Title of the section.
    pub title: T,
    /// ID used to reference it.
    pub id: Option<Id>,
    /// Body of the section.
    pub body: B,
    /// List of child sections, can be an array, a vector, or anything that
    /// iterates by ref.
    pub children: L,
}

impl<T, B, L> fmt::Debug for Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_struct("Section");
        debug_fmtr
            .field("title", &self.title)
            .field("id", &self.id)
            .field("body", &self.body);
        for (i, element) in self.children.iter().enumerate() {
            debug_fmtr.field(&i.to_string(), &element);
        }
        debug_fmtr.finish()
    }
}

impl<T, B, L> Clone for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Clone,
    B: Component<Kind = BlockComponent> + Clone,
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent>,
{
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            id: self.id.clone(),
            body: self.body.clone(),
            children: self.children.clone(),
        }
    }
}

impl<T, B, L> PartialEq for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + PartialEq,
    B: Component<Kind = BlockComponent> + PartialEq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.body == other.body
            && self.children.iter().eq(other.children.iter())
    }
}

impl<T, B, L> Eq for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Eq,
    B: Component<Kind = BlockComponent> + Eq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Eq,
{
}

impl<T, B, L> PartialOrd for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + PartialOrd,
    B: Component<Kind = BlockComponent> + PartialOrd,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self
            .title
            .partial_cmp(&other.title)?
            .then(self.body.partial_cmp(&other.body)?)
            .then(self.children.iter().partial_cmp(other.children.iter())?);
        Some(ordering)
    }
}

impl<T, B, L> Ord for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Ord,
    B: Component<Kind = BlockComponent> + Ord,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.title
            .cmp(&other.title)
            .then_with(|| self.body.cmp(&other.body))
            .then_with(|| self.children.iter().cmp(other.children.iter()))
    }
}

impl<T, B, L> Hash for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Hash,
    B: Component<Kind = BlockComponent> + Hash,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.title.hash(state);
        self.body.hash(state);
        for (i, child) in self.children.iter().enumerate() {
            i.hash(state);
            child.hash(state);
        }
    }
}

impl<T, B, L> Default for Section<T, B, L>
where
    T: Component<Kind = InlineComponent> + Default,
    B: Component<Kind = BlockComponent> + Default,
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Hash,
{
    fn default() -> Self {
        Self {
            title: T::default(),
            id: Option::default(),
            body: B::default(),
            children: L::default(),
        }
    }
}

impl<T, B, L> Component for Section<T, B, L>
where
    T: Component<Kind = InlineComponent>,
    B: Component<Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = SectionComponent> + Hash,
{
    type Kind = SectionComponent;
}

impl<T, B, L> Render<Html> for Section<T, B, L>
where
    T: Render<Html, Kind = InlineComponent>,
    B: Render<Html, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = SectionComponent> + Hash,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let tag = match ctx.section_level() {
            0 => "h2",
            1 => "h3",
            2 => "h4",
            3 => "h5",
            _ => "h6",
        };
        write!(
            renderer,
            "<div class=\"pedia-section pedia-section-{}\"",
            ctx.section_level()
        )?;
        if let Some(id) = &self.id {
            renderer.write_str(" id=\"")?;
            id.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\"")?;
        }
        write!(renderer, "><{} class=\"pedia-title\">", tag)?;
        if let Some(id) = &self.id {
            let location = Location::Internal(InternalLoc {
                path: ctx.location().clone(),
                id: Some(id.clone()),
            });
            renderer.write_str("<a class=\"pedia-title-link\" href=\"")?;
            location.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\">")?;
        }
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        if self.id.is_some() {
            renderer.write_str("</a>")?;
        }
        write!(
            renderer,
            "</{}><div class=\"pedia-section-body-wrapper\"><div \
             class=\"pedia-section-body\">",
            tag
        )?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        renderer.write_str("</div><div class=\"pedia-section-children\">")?;
        for child in self.children.iter() {
            child.render(
                renderer,
                ctx.enter_section().with_kind(&SectionComponent),
            )?;
        }
        renderer.write_str("</div></div></div>")?;
        Ok(())
    }
}

impl<T, B, L> Render<Markdown> for Section<T, B, L>
where
    T: Render<Markdown, Kind = InlineComponent>,
    B: Render<Markdown, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = SectionComponent> + Hash,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        let tag = match ctx.section_level() {
            0 => "##",
            1 => "###",
            2 => "####",
            3 => "#####",
            _ => "######",
        };
        write!(renderer, "{} ", tag)?;
        if let Some(id) = &self.id {
            renderer.write_str("<span id=\"")?;
            id.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str("\">[")?;
        }

        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;

        if let Some(id) = &self.id {
            let location = Location::Internal(InternalLoc {
                path: ctx.location().clone(),
                id: Some(id.clone()),
            });
            renderer.write_str("](")?;
            location.render(renderer, ctx.with_kind(&InlineComponent))?;
            renderer.write_str(")")?;
        }
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in self.children.iter() {
            child.render(
                renderer,
                ctx.enter_section().with_kind(&SectionComponent),
            )?;
        }
        Ok(())
    }
}

impl<T, B, L> Render<Text> for Section<T, B, L>
where
    T: Render<Text, Kind = InlineComponent>,
    B: Render<Text, Kind = BlockComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = SectionComponent> + Hash,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.title.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n\n")?;
        self.body.render(renderer, ctx.with_kind(&BlockComponent))?;
        for child in self.children.iter() {
            child.render(
                renderer,
                ctx.enter_section().with_kind(&SectionComponent),
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Section, SectionComponent};
    use crate::{
        component::block::text::Paragraph,
        harray,
        location::{Id, InternalPath},
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn section_with_id_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section {
                title: "Hello",
                id: Some(Id::new("hello")),
                body: Paragraph("World!"),
                children: harray![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn section_without_id_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section {
                title: "Hello",
                id: None,
                body: Paragraph("World!"),
                children: harray![],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn section_with_children_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Section {
                title: "Hello",
                id: None,
                body: Paragraph("World!"),
                children: harray![
                    Section {
                        title: "Hey",
                        id: None,
                        body: Paragraph("Hey!"),
                        children: harray![],
                    },
                    Section {
                        title: "Good",
                        id: Some(Id::new("good")),
                        body: Paragraph("Afternoon!"),
                        children: harray![Section {
                            title: "By",
                            id: None,
                            body: Paragraph("Bye!"),
                            children: harray![],
                        }],
                    },
                    Section {
                        title: "Hay",
                        id: None,
                        body: Paragraph("Bay!"),
                        children: harray![],
                    },
                ],
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &SectionComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
