use super::{buf::DefaultOwnedStr, fragment::Fragment, internal::Internal};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location<FS = Vec<Fragment>, I = DefaultOwnedStr, E = Url> {
    Internal(Internal<FS, I>),
    External(E),
}
