# Codedpedia

Codedpedia is a static site generator designed for static encyclopedias that are
programmed in Rust. The project can be resumed in a few items:
- Components;
- Component kinds;
- Rendering formats;
- Sites.

Components implement the `Component` trait and have a `ComponentKind`
associated with it that serves for the purpose of typing components. Codedpedia's
builtin component kinds are:
- `InlineComponent` - components inlined in text;
- `BlockComponent` - components with blocks of their own;
- `table::CellComponent` - cells of a table;
- `table::RowComponent` - rows of a table;
- `SectionComponent` - sections of an article/page;
- `AssetComponent` - asset of an article/page;
- `PageComponent` - the whole article/page.

Rendering formats are a proxy to Rust's `fmt` which allows specific formatting
e.g. in a specific context, such as in a nested list. More important than that
is that they also are used to define what kind of output the rendering will
produce. Builtin rendering formats are:
- HTML;
- Markdown;
- Plaintext.

Components can implement a trait named `Render<W>` where `W` is a rendering
format. For instance, if a component wants to render HTML, it must implement
`Render<Html>`.

Finally, a `Site` is a collection of pages structured in terms of in-memory
directories. The build process consists of generating pages into actual
directories, as well copying resources. Look at the function `static_site_main`.
No JavaScript dependency required for the build output.

# Documentation

https://brunoczim.github.io/codedpedia/codedpedia/

# Example

```rust
use codedpedia::{
    component::{
        asset::{AssetComponent, Stylesheet},
        block::{list::UnorderedList, text::Paragraph, InlineBlock},
        inline::text::Link,
        page::{Page, PageComponent},
        section::Section,
        BlockComponent,
    },
    harray,
    location::{Id, InternalPath, Location},
    render::{DynFullComponent, FullRender, Html, Render},
    site::{Entry, Site},
    static_site_main,
};

fn default_assets(
) -> impl Render<Html, Kind = AssetComponent> + Send + Sync + 'static {
    [Stylesheet { location: Location::internal("styles/main.css") }]
}

fn banner() -> impl FullRender<Kind = BlockComponent> + Send + Sync + 'static {
    InlineBlock(Link {
        target: "Simple Pedia",
        location: Location::internal(""),
    })
}

fn index() -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
    Page {
        banner: banner(),
        title: String::from("Simple Pedia"),
        assets: default_assets(),
        body: harray![
            Paragraph(
                "This is the initial page of the simple pedia. You can dive \
                 down into the following:"
            ),
            UnorderedList(harray![
                InlineBlock(Link {
                    location: Location::internal("foo"),
                    target: "Foo stuff",
                }),
                InlineBlock(Link {
                    location: Location::internal("bar"),
                    target: "Bar stiff",
                }),
            ]),
        ],
        children: harray![
            Section {
                title: "Random Section",
                id: Some(Id::new("random").unwrap()),
                body: Paragraph("This is a random paragraph."),
                children: harray![
                    Section {
                        title: "Randomly First",
                        id: Some(Id::new("random-first").unwrap()),
                        body: Paragraph(
                            "This the first (really?) random paragraph."
                        ),
                        children: harray![],
                    },
                    Section {
                        title: "Randomly Second",
                        id: Some(Id::new("random-second").unwrap()),
                        body: Paragraph(
                            "This the second (really??) random paragraph."
                        ),
                        children: harray![],
                    }
                ],
            },
            Section {
                title: "Weird Title",
                id: Some(Id::new("weird").unwrap()),
                body: Paragraph("Weird paragraph as an example"),
                children: harray![],
            }
        ],
    }
}

fn foo_page() -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
    Page {
        banner: banner(),
        title: String::from("Foo"),
        assets: default_assets(),
        body: harray![Paragraph("Foo is a metavariable."),],
        children: harray![],
    }
}

fn bar_page() -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
    Page {
        banner: banner(),
        title: String::from("Bar"),
        assets: default_assets(),
        body: harray![Paragraph(harray![
            "Bar is a metavariable. ",
            Link { location: Location::internal("bar/baz"), target: "Baz" },
            " is also a metavariable."
        ])],
        children: harray![],
    }
}

fn baz_page() -> impl FullRender<Kind = PageComponent> + Send + Sync + 'static {
    Page {
        banner: banner(),
        title: String::from("Baz"),
        assets: default_assets(),
        body: harray![Paragraph(harray![
            "Baz is a metavariable, similar to ",
            Link { location: Location::internal("bar"), target: "Bar" },
            "."
        ])],
        children: harray![],
    }
}

fn simple_pedia_site() -> Site<DynFullComponent<'static, PageComponent>> {
    let mut site = Site::default();
    site.root.insert_path(
        &InternalPath::parse("index.html").unwrap(),
        Entry::Page(index().into_dyn()),
    );
    site.root.insert_path(
        &InternalPath::parse("foo/index.html").unwrap(),
        Entry::Page(foo_page().into_dyn()),
    );
    site.root.insert_path(
        &InternalPath::parse("bar/index.html").unwrap(),
        Entry::Page(bar_page().into_dyn()),
    );
    site.root.insert_path(
        &InternalPath::parse("bar/baz/index.html").unwrap(),
        Entry::Page(baz_page().into_dyn()),
    );
    site.root.insert_path(
        &InternalPath::parse("styles/main.css").unwrap(),
        Entry::Resource,
    );
    site
}

fn main() {
    let site = simple_pedia_site();

    static_site_main(&site, &mut Html, "examples/build", "examples/assets");
}
```
