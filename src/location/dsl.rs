use super::{
    component::{Component, InvalidComponent},
    external::{External, InvalidExternal},
    general::{InvalidLocation, Location},
    id::{Id, InvalidId},
    internal::{Internal, InvalidInternal},
    path::{InvalidPath, Path},
};

pub trait LocationStrExt {
    fn try_id(&self) -> Result<&Id, InvalidId>;

    fn try_component(&self) -> Result<&Component, InvalidComponent>;

    fn try_path(&self) -> Result<&Path, InvalidPath>;

    fn try_internal_loc(&self) -> Result<&Internal, InvalidInternal>;

    fn try_external_loc(&self) -> Result<&External, InvalidExternal>;

    fn try_location(&self) -> Result<&Location, InvalidLocation>;

    fn try_into_id(self: Box<Self>) -> Result<Box<Id>, InvalidId>;

    fn try_into_component(
        self: Box<Self>,
    ) -> Result<Box<Component>, InvalidComponent>;

    fn try_into_path(self: Box<Self>) -> Result<Box<Path>, InvalidPath>;

    fn try_into_internal_loc(
        self: Box<Self>,
    ) -> Result<Box<Internal>, InvalidInternal>;

    fn try_into_external_loc(
        self: Box<Self>,
    ) -> Result<Box<External>, InvalidExternal>;

    fn try_into_location(
        self: Box<Self>,
    ) -> Result<Box<Location>, InvalidLocation>;

    fn id(&self) -> &Id {
        self.try_id().expect("failed to parse id")
    }

    fn component(&self) -> &Component {
        self.try_component().expect("failed to parse path component")
    }

    fn path(&self) -> &Path {
        self.try_path().expect("failed to parse path")
    }

    fn internal_loc(&self) -> &Internal {
        self.try_internal_loc().expect("failed to parse internal location")
    }

    fn external_loc(&self) -> &External {
        self.try_external_loc().expect("failed to parse external location")
    }

    fn location(&self) -> &Location {
        self.try_location().expect("failed to parse location")
    }

    fn into_id(self: Box<Self>) -> Box<Id> {
        self.try_into_id().expect("failed to parse into id")
    }

    fn into_component(self: Box<Self>) -> Box<Component> {
        self.try_into_component().expect("failed to parse into path component")
    }

    fn into_path(self: Box<Self>) -> Box<Path> {
        self.try_into_path().expect("failed to parse into path")
    }

    fn into_internal_loc(self: Box<Self>) -> Box<Internal> {
        self.try_into_internal_loc()
            .expect("failed to parse into internal location")
    }

    fn into_external_loc(self: Box<Self>) -> Box<External> {
        self.try_into_external_loc()
            .expect("failed to parse into external location")
    }

    fn into_location(self: Box<Self>) -> Box<Location> {
        self.try_into_location().expect("failed to parse into location")
    }
}

impl LocationStrExt for str {
    fn try_id(&self) -> Result<&Id, InvalidId> {
        Id::parse(self)
    }

    fn try_component(&self) -> Result<&Component, InvalidComponent> {
        Component::parse(self)
    }

    fn try_path(&self) -> Result<&Path, InvalidPath> {
        Path::parse(self)
    }

    fn try_internal_loc(&self) -> Result<&Internal, InvalidInternal> {
        Internal::parse(self)
    }

    fn try_external_loc(&self) -> Result<&External, InvalidExternal> {
        External::parse(self)
    }

    fn try_location(&self) -> Result<&Location, InvalidLocation> {
        Location::parse(self)
    }

    fn try_into_id(self: Box<Self>) -> Result<Box<Id>, InvalidId> {
        Id::parse_boxed(self)
    }

    fn try_into_component(
        self: Box<Self>,
    ) -> Result<Box<Component>, InvalidComponent> {
        Component::parse_boxed(self)
    }

    fn try_into_path(self: Box<Self>) -> Result<Box<Path>, InvalidPath> {
        Path::parse_boxed(self)
    }

    fn try_into_internal_loc(
        self: Box<Self>,
    ) -> Result<Box<Internal>, InvalidInternal> {
        Internal::parse_boxed(self)
    }

    fn try_into_external_loc(
        self: Box<Self>,
    ) -> Result<Box<External>, InvalidExternal> {
        External::parse_boxed(self)
    }

    fn try_into_location(
        self: Box<Self>,
    ) -> Result<Box<Location>, InvalidLocation> {
        Location::parse_boxed(self)
    }
}
