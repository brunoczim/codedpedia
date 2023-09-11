use std::error::Error;

pub trait Parse<'input>: Sized {
    type Error: Error;

    fn parse(input: &'input str) -> Result<Self, Self::Error>;
}
