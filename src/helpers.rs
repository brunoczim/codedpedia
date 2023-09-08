#[macro_export]
macro_rules! decl_page {
    ($($vis:vis fn $name:ident () -> _ $impl:block)*) => {
        $(
            $vis fn $name()
                -> impl $crate::render::FullRender<
                    Kind = $crate::component::page::PageComponent
                > + Send + Sync + 'static
            {
                $impl
            }
        )*
    };
}
