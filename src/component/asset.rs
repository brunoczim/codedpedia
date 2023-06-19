//! This module exports page assets components.

use super::{Component, ComponentKind, InlineComponent};
use crate::{
    location::Location,
    render::{Context, Html, Render, Renderer},
};
use std::fmt::{self, Write};

/// The kind of an asset component. Such component is an external resource vital
/// to an encyclopedia presentation or functioning as opposed to optional (e.g.
/// images).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AssetComponent;

impl ComponentKind for AssetComponent {}

/// Stylesheet asset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Stylesheet {
    /// Location to the stylesheet.
    pub location: Location,
}

impl Component for Stylesheet {
    type Kind = AssetComponent;
}

impl Render<Html> for Stylesheet {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<link rel=\"stylesheet\" href=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\">")?;
        Ok(())
    }
}

/// JavaScript script asset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Script {
    /// Location to the script.
    pub location: Location,
}

impl Component for Script {
    type Kind = AssetComponent;
}

impl Render<Html> for Script {
    fn render(
        &self,
        renderer: &mut Renderer<Html>,
        ctx: Context<Self::Kind>,
    ) -> fmt::Result {
        renderer.write_str("<script type=\"application/javascript\" src=\"")?;
        self.location.render(renderer, ctx.with_kind(&InlineComponent))?;
        renderer.write_str("\"></script>")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{AssetComponent, Script, Stylesheet};
    use crate::{
        location::{InternalPath, Location},
        render::{
            html::test::validate_html_fragment,
            Context,
            Html,
            RenderAsDisplay,
        },
    };

    #[test]
    fn stylesheet_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Stylesheet { location: Location::internal("styles/main.css") },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &AssetComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }

    #[test]
    fn script_is_valid_html() {
        let rendered = RenderAsDisplay::new(
            Script { location: Location::internal("js/main.js") },
            &mut Html::default(),
            Context::new(&InternalPath::default(), &AssetComponent),
        )
        .to_string();

        validate_html_fragment(&rendered).unwrap();
    }
}
