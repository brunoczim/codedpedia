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
    site.root
        .insert_index(InternalPath::root(), Entry::Page(index().into_dyn()));
    site.root.insert_index(
        InternalPath::parse("foo").unwrap(),
        Entry::Page(foo_page().into_dyn()),
    );
    site.root.insert_index(
        InternalPath::parse("bar").unwrap(),
        Entry::Page(bar_page().into_dyn()),
    );
    site.root.insert_index(
        InternalPath::parse("bar/baz").unwrap(),
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
