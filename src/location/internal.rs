use super::{id::AsId, path::AsPath};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Internal<I, P>
where
    I: AsId,
    P: AsPath,
{
    Id(I),
    Path(P),
    PathWithId(P, I),
}
