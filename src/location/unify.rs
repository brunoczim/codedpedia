use super::{
    buf::{DefaultOwnedStr, TextBuf},
    external::ExternalUrl,
    fragment::{Fragment, FragmentRef},
    general::Location,
    id::{Id, IdRef},
    internal::Internal,
    path::{Path, PathFragments, PathRef},
};
use url::Url;

pub trait ToLocation: Sized {
    type External: ExternalUrl;
    type IdBuf: TextBuf;
    type FragmentBuf: TextBuf;
    type PathFragments: PathFragments<FragmentBuf = Self::FragmentBuf>;

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External>;

    fn to_owned_location(self) -> Location {
        match self.to_location() {
            Location::External(external) => {
                Location::External(external.into_url())
            },
            Location::Internal(Internal::Id(id)) => {
                Location::Internal(Internal::Id(id.into_default_owned()))
            },
            Location::Internal(Internal::Path(path)) => {
                Location::Internal(Internal::Path(Path {
                    fragments: path.fragments.into_fragment_vec(),
                }))
            },
            Location::Internal(Internal::PathWithId(path, id)) => {
                Location::Internal(Internal::PathWithId(
                    Path { fragments: path.fragments.into_fragment_vec() },
                    id.into_default_owned(),
                ))
            },
        }
    }
}

impl<FS, F, I, E> ToLocation for Location<FS, I, E>
where
    F: TextBuf,
    FS: PathFragments<FragmentBuf = F>,
    I: TextBuf,
    E: ExternalUrl,
{
    type External = E;
    type IdBuf = I;
    type FragmentBuf = F;
    type PathFragments = FS;

    fn to_location(
        self,
    ) -> Location<
        Self::PathFragments,
        Self::IdBuf,
        <Self as ToLocation>::External,
    > {
        self
    }
}

pub type LocactionRef<'path, 'id, 'ext, F = Fragment> =
    Location<PathRef<'path, F>, IdRef<'id>, &'ext Url>;

pub type LocactionFragmentsRef<'path, 'frag, 'id, 'ext> =
    LocactionRef<'path, 'id, 'ext, FragmentRef<'frag>>;

impl<E> ToLocation for E
where
    E: ExternalUrl,
{
    type External = E;
    type IdBuf = DefaultOwnedStr;
    type FragmentBuf = DefaultOwnedStr;
    type PathFragments = Vec<Fragment<Self::FragmentBuf>>;

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External> {
        Location::External(self)
    }
}

impl<FS, F, I> ToLocation for Internal<FS, I>
where
    F: TextBuf,
    FS: PathFragments<FragmentBuf = F>,
    I: TextBuf,
{
    type External = Url;
    type IdBuf = I;
    type FragmentBuf = F;
    type PathFragments = FS;

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External> {
        Location::Internal(self)
    }
}

impl<FS, F> ToLocation for Path<FS>
where
    F: TextBuf,
    FS: PathFragments<FragmentBuf = F>,
{
    type External = Url;
    type IdBuf = DefaultOwnedStr;
    type FragmentBuf = F;
    type PathFragments = FS;

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External> {
        Location::Internal(Internal::Path(self))
    }
}

impl<S> ToLocation for Fragment<S>
where
    S: TextBuf,
{
    type External = Url;
    type IdBuf = DefaultOwnedStr;
    type FragmentBuf = S;
    type PathFragments = [Fragment<Self::FragmentBuf>; 1];

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External> {
        Location::Internal(Internal::Path(Path { fragments: [self] }))
    }
}

impl<S> ToLocation for Id<S>
where
    S: TextBuf,
{
    type External = Url;
    type IdBuf = S;
    type FragmentBuf = DefaultOwnedStr;
    type PathFragments = Vec<Fragment<Self::FragmentBuf>>;

    fn to_location(
        self,
    ) -> Location<Self::PathFragments, Self::IdBuf, Self::External> {
        Location::Internal(Internal::Id(self))
    }
}
