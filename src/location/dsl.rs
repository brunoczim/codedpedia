use super::{
    component::{Component, InvalidComponent},
    id::{Id, InvalidId},
    path::{InvalidPath, Path},
};

pub trait LocationStrExt {
    fn try_id(&self) -> Result<&Id, InvalidId>;

    fn try_component(&self) -> Result<&Component, InvalidComponent>;

    fn try_path(&self) -> Result<&Path, InvalidPath>;

    fn try_into_id(self: Box<Self>) -> Result<Box<Id>, InvalidId>;

    fn try_into_component(
        self: Box<Self>,
    ) -> Result<Box<Component>, InvalidComponent>;

    fn try_into_path(self: Box<Self>) -> Result<Box<Path>, InvalidPath>;

    fn id(&self) -> &Id {
        self.try_id().expect("failed to parse id")
    }

    fn component(&self) -> &Component {
        self.try_component().expect("failed to parse path component")
    }

    fn path(&self) -> &Path {
        self.try_path().expect("failed to parse path")
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
}
