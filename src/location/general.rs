use super::{external::AsExternal, id::AsId, internal::Internal, path::AsPath};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location<I, P, E>
where
    I: AsId,
    P: AsPath,
    E: AsExternal,
{
    Internal(Internal<I, P>),
    External(E),
}
