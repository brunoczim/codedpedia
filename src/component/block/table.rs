//! This module exports a table component.

use crate::{
    component::{
        block::BlockComponent,
        Component,
        ComponentKind,
        InlineComponent,
    },
    hseq::IntoIterRef,
    render::{Context, Html, Markdown, Render, Renderer, Text},
};
use std::{
    cmp::Ordering,
    fmt,
    fmt::Write,
    hash::{Hash, Hasher},
};

/// Cell component kind. Components of this kind are usable as cells in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CellComponent;

impl ComponentKind for CellComponent {}

/// Row component kind. Components of this kind are usable as rows in a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RowComponent;

impl ComponentKind for RowComponent {}

/// Attributes of a table cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellAttrs {
    /// Is this cell a header?
    pub header: bool,
    /// Span size in the row direction of this cell.
    pub rowspan: u32,
    /// Span size in the column direction of this cell.
    pub colspan: u32,
}

impl Default for CellAttrs {
    fn default() -> Self {
        Self { header: false, rowspan: 1, colspan: 1 }
    }
}

/// A table cell, that can be a header, and can span over multiple columns or
/// rows.
///
/// # HTML Classes
///
/// - `pedia-table-header` attached to a `<th>` element if header.
/// - `pedia-table-cell` attached to a `<td>` element if regular cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    /// Child component displayed in the cell.
    pub child: T,
    /// Cell attributes, such as span and declaring it as header or not.
    pub attrs: CellAttrs,
}

impl<T> From<T> for Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    fn from(child: T) -> Self {
        Self { child, attrs: CellAttrs::default() }
    }
}

impl<T> Component for Cell<T>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = CellComponent;
}

impl<T> Render<Html> for Cell<T>
where
    T: Render<Html, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        if self.attrs.header {
            write!(renderer, "<th class=\"pedia-table-header\"")?;
        } else {
            write!(renderer, "<td class=\"pedia-table-cell\"")?;
        }
        if self.attrs.rowspan != 1 {
            write!(renderer, " rowspan=\"{}\"", self.attrs.rowspan)?;
        }
        if self.attrs.colspan != 1 {
            write!(renderer, " colspan=\"{}\"", self.attrs.colspan)?;
        }
        write!(renderer, ">")?;
        self.child.render(renderer, ctx.with_kind(&BlockComponent))?;
        if self.attrs.header {
            write!(renderer, "</th>")?;
        } else {
            write!(renderer, "</td>")?;
        }
        Ok(())
    }
}

impl<T> Render<Markdown> for Cell<T>
where
    T: Render<Markdown, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        if self.attrs.header {
            write!(renderer, "<th")?;
        } else {
            write!(renderer, "<td")?;
        }
        if self.attrs.rowspan != 1 {
            write!(renderer, " rowspan=\"{}\"", self.attrs.rowspan)?;
        }
        if self.attrs.colspan != 1 {
            write!(renderer, " colspan=\"{}\"", self.attrs.colspan)?;
        }
        write!(renderer, ">")?;
        self.child.render(renderer, ctx.with_kind(&BlockComponent))?;
        if self.attrs.header {
            write!(renderer, "</th>")?;
        } else {
            write!(renderer, "</td>")?;
        }
        Ok(())
    }
}

impl<T> Render<Text> for Cell<T>
where
    T: Render<Text, Kind = BlockComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> std::fmt::Result {
        self.child.render(renderer, ctx.with_kind(&BlockComponent))
    }
}

/// A row in a table. The single unnamed field could be an array, a vec, or
/// anything that iterates by ref yielding cell components.
///
/// # HTML Classes
///
/// - `pedia-table-row` attached to a `<tr>` element.
pub struct Row<C>(pub C)
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>;

impl<C> fmt::Debug for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("Row");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<C> Clone for Row<C>
where
    C: IntoIterRef + Clone,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C> Copy for Row<C>
where
    C: IntoIterRef + Copy,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
}

impl<C> PartialEq for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<C> Eq for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Eq,
{
}

impl<C> PartialOrd for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<C> Ord for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<C> Hash for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent> + Hash,
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

impl<C> Default for Row<C>
where
    C: IntoIterRef + Default,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    fn default() -> Self {
        Self(C::default())
    }
}

impl<C> Component for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Component<Kind = CellComponent>,
{
    type Kind = RowComponent;
}

impl<C> Render<Html> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Html, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<tr class=\"pedia-table-row\">")?;
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
        }
        renderer.write_str("</tr>")?;
        Ok(())
    }
}

impl<C> Render<Markdown> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Markdown, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<tr>")?;
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
        }
        renderer.write_str("</tr>")?;
        Ok(())
    }
}

impl<C> Render<Text> for Row<C>
where
    C: IntoIterRef,
    <C as IntoIterRef>::Item: Render<Text, Kind = CellComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for cell in self.0.iter() {
            cell.render(renderer, ctx.with_kind(&CellComponent))?;
            renderer.write_str("\n")?;
        }
        Ok(())
    }
}

/// A table. The single unnamed field could be an array, a vec, or
/// anything that iterates by ref yielding row components.
///
/// # HTML Classes
///
/// - `pedia-table` attached to a `<div>` element surrounding a `<table>`
///   element.
pub struct Table<L>(pub L)
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>;

impl<L> fmt::Debug for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_fmtr = fmtr.debug_tuple("Table");
        for element in self.0.iter() {
            debug_fmtr.field(&element);
        }
        debug_fmtr.finish()
    }
}

impl<L> Clone for Table<L>
where
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<L> Copy for Table<L>
where
    L: IntoIterRef + Copy,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
}

impl<L> PartialEq for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().eq(other.0.iter())
    }
}

impl<L> Eq for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Eq,
{
}

impl<L> PartialOrd for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.iter().partial_cmp(other.0.iter())
    }
}

impl<L> Ord for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl<L> Hash for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Hash,
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

impl<L> Default for Table<L>
where
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn default() -> Self {
        Self(L::default())
    }
}

impl<L> Component for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    type Kind = BlockComponent;
}

impl<L> Render<Html> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"pedia-table\"><table>")?;
        for row in self.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table></div>")?;
        Ok(())
    }
}

impl<L> Render<Markdown> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<table>")?;
        for row in self.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table>")?;
        Ok(())
    }
}

impl<L> Render<Text> for Table<L>
where
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        for row in self.0.iter() {
            renderer.write_str("- - - - - - - - - - -\n\n")?;
            row.render(renderer, ctx.with_kind(&RowComponent))?;
            renderer.write_str("\n")?;
        }
        Ok(())
    }
}

/// A table with a title/caption at the top.
pub struct CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    /// The caption of this table.
    pub caption: C,
    /// The table itself.
    pub table: Table<L>,
}

impl<C, L> fmt::Debug for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("CaptionedTable")
            .field("caption", &self.caption)
            .field("table", &self.table)
            .finish()
    }
}

impl<C, L> Clone for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Clone,
    L: IntoIterRef + Clone,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn clone(&self) -> Self {
        Self { caption: self.caption.clone(), table: self.table.clone() }
    }
}

impl<C, L> Copy for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Copy,
    L: IntoIterRef + Copy,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
}

impl<C, L> PartialEq for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + PartialEq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.caption == other.caption && self.table == other.table
    }
}

impl<C, L> Eq for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Eq,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Eq,
{
}

impl<C, L> PartialOrd for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + PartialOrd,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.caption
                .partial_cmp(&other.caption)?
                .then(self.table.partial_cmp(&other.table)?),
        )
    }
}

impl<C, L> Ord for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Ord,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.caption
            .cmp(&other.caption)
            .then_with(|| self.table.cmp(&other.table))
    }
}

impl<C, L> Hash for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Hash,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.caption.hash(state);
        self.table.hash(state);
    }
}

impl<C, L> Default for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent> + Default,
    L: IntoIterRef + Default,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    fn default() -> Self {
        Self { caption: C::default(), table: Table::default() }
    }
}

impl<C, L> Component for CaptionedTable<C, L>
where
    C: Component<Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Component<Kind = RowComponent>,
{
    type Kind = BlockComponent;
}

impl<C, L> Render<Html> for CaptionedTable<C, L>
where
    C: Render<Html, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Html, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<div class=\"pedia-table\"><table><caption>")?;
        self.caption.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</caption>")?;
        for row in self.table.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table></div>")?;
        Ok(())
    }
}

impl<C, L> Render<Markdown> for CaptionedTable<C, L>
where
    C: Render<Markdown, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Markdown, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Markdown>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<table><caption>")?;
        self.caption.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("</caption>")?;
        for row in self.table.0.iter() {
            row.render(renderer, ctx.with_kind(&RowComponent))?;
        }
        renderer.write_str("</table>")?;
        Ok(())
    }
}

impl<C, L> Render<Text> for CaptionedTable<C, L>
where
    C: Render<Text, Kind = InlineComponent>,
    L: IntoIterRef,
    <L as IntoIterRef>::Item: Render<Text, Kind = RowComponent>,
{
    fn render(
        &self,
        renderer: &mut Renderer<Text>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        self.caption.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\n")?;
        self.table.render(renderer, ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{CaptionedTable, Cell, CellAttrs, Row, Table};
    use crate::{
        component::{
            block::{text::Paragraph, InlineBlock},
            inline::text::Bold,
            BlockComponent,
        },
        harray,
        location::InternalPath,
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn table_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Table(harray![
                Row(harray![
                    Cell {
                        child: InlineBlock("abc"),
                        attrs: CellAttrs::default()
                    },
                    Cell {
                        child: Paragraph("123"),
                        attrs: CellAttrs::default()
                    },
                ]),
                Row(harray![Cell {
                    child: Paragraph("a c r m f m"),
                    attrs: CellAttrs::default()
                }])
            ]),
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn titled_table_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            CaptionedTable {
                caption: Bold("aaaaaaa"),
                table: Table(harray![
                    Row(harray![
                        Cell {
                            child: InlineBlock("abc"),
                            attrs: CellAttrs::default()
                        },
                        Cell {
                            child: Paragraph("123"),
                            attrs: CellAttrs::default()
                        },
                    ]),
                    Row(harray![Cell {
                        child: Paragraph("a c r m f m"),
                        attrs: CellAttrs::default()
                    }])
                ]),
            },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &BlockComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
