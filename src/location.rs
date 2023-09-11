use url::Url;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location<I, P, U> {
    Internal(Internal<I, P>),
    External(U),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Internal<I, P> {
    Id(I),
    Path(P),
    PathWithId(P, I),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Path<C>
where
    C: AsRef<Component>,
{
    pub components: [C],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Component {
    contents: str,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id {
    contents: str,
}
