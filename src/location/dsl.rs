use super::{
    component::InvalidComponent,
    id::{Id, InvalidId},
};

pub trait StrExt {
    fn try_id(&self) -> Result<&Id, InvalidId>;

    fn try_component(&self) -> Result<&Component, InvalidComponent>;

    fn try_into_id(self: Box<Self>) -> Result<Box<Id>, InvalidId>;

    fn try_into_component(
        self: Box<Self>,
    ) -> Result<Box<Component>, InvalidComponent>;

    fn id(&self) -> &Id {
        self.try_id().expect("failed to parse id")
    }

    fn component(&self) -> &Component {
        self.try_component().expect("failed to parse path component")
    }

    fn into_id(self: Box<Self>) -> Box<Id> {
        self.try_into_id().expect("failed to parse into id")
    }

    fn into_component(self: Box<Self>) -> Box<Component> {
        self.try_into_component().expect("failed to parse into path component")
    }
}

impl StrExt for str {
    fn try_id(&self) -> Result<&Id, InvalidId> {
        Id::parse(self)
    }

    fn try_into_id(self: Box<Self>) -> Result<Box<Id>, InvalidId> {
        Id::parse_owned(self)
    }

    fn try_component(&self) -> Result<&Component, InvalcomponentComponent> {
        Component::parse(self)
    }

    fn try_into_component(
        self: Box<Self>,
    ) -> Result<Box<Component>, InvalcomponentComponent> {
        Component::parse_owned(self)
    }
}
