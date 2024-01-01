//! This module exports list related components.

use crate::{
    domain::{
        component::Component,
        render::{self, Render, Renderer},
    },
    format::{markdown, text, Html, Markdown, Text},
    sequence::Sequence,
};

use super::BlockComponent;
use std::{
    cmp::Ordering,
    fmt::{self, Write},
    hash::{Hash, Hasher},
};

pub struct UnorderedList<L>(pub L)
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>;

impl<L> fmt::Debug for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("UnorderedList");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<L> Clone for UnorderedList<L>
where
    L: Sequence + Clone,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for UnorderedList<L>
where
    L: Sequence + Copy,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
}

impl<L> PartialEq for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<L> Eq for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Eq,
{
}

impl<L> PartialOrd for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<L> Ord for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<L> Hash for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (i, element) in self.0.iter().enumerate() {
            i.hash(state);
            element.hash(state);
        }
    }
}

impl<L> Default for UnorderedList<L>
where
    L: Sequence + Default,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn default() -> Self {
        Self(L::default())
    }
}

impl<L> Component for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<ul class=\"pedia-unord-list\">")?;
        for element in self.0.iter() {
            renderer.write_str("<li class=\"pedia-list-elem\">")?;
            element.render(renderer, ctx)?;
            renderer.write_str("</li>")?;
        }
        renderer.write_str("</ul>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(markdown::Nest, |renderer| {
            for element in self.0.iter() {
                renderer.write_str("-")?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

impl<L> Render<Text> for UnorderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(text::Nest, |renderer| {
            for element in self.0.iter() {
                renderer.write_str("-")?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

pub struct OrderedList<L>(pub L)
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>;

impl<L> fmt::Debug for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("OrderedList");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<L> Clone for OrderedList<L>
where
    L: Sequence + Clone,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for OrderedList<L>
where
    L: Sequence + Copy,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
}

impl<L> PartialEq for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<L> Eq for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Eq,
{
}

impl<L> PartialOrd for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<L> Ord for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<L> Hash for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        for (i, element) in self.0.iter().enumerate() {
            i.hash(state);
            element.hash(state);
        }
    }
}

impl<L> Default for OrderedList<L>
where
    L: Sequence + Default,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    fn default() -> Self {
        Self(L::default())
    }
}

impl<L> Component for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Component<Kind = BlockComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<ol class=\"pedia-ord-list\">")?;
        for element in self.0.iter() {
            renderer.write_str("<li class=\"pedia-list-elem\">")?;
            element.render(renderer, ctx)?;
            renderer.write_str("</li>")?;
        }
        renderer.write_str("</ol>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(markdown::Nest, |renderer| {
            for element in self.0.iter() {
                renderer.write_str("-")?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

impl<L> Render<Text> for OrderedList<L>
where
    L: Sequence,
    <L as Sequence>::Item: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: render::Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.scoped(text::Nest, |renderer| {
            for (i, element) in self.0.iter().enumerate() {
                write!(renderer, "{}. ", i)?;
                element.render(renderer, ctx)?;
                renderer.write_str("\n")?;
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use crate::domain::render::RenderAsDisplay;

    use super::{OrderedList, UnorderedList};

    #[test]
    fn unordered_list_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            UnorderedList(harray![
                (InlineBlock("abc"), Paragraph("def")): BlockComponent
            ]),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn ordered_list_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            OrderedList(harray![
                (InlineBlock("abc"), Paragraph("def")): BlockComponent
            ]),
            &mut Html::default(),
            render::Context::new(location::Path::ROOT, &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
