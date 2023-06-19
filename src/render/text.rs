//! This module provides utilities about the plaintext rendering format.

use super::{
    common_text::{self, CommonText},
    Format,
    Scope,
};
use std::fmt;

/// The plaintext rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Text {
    inner: CommonText,
}

impl Text {
    /// Creates a new plaintext format renderer given indentation size in number
    /// of spaces.
    pub fn new(indent_size: u32) -> Self {
        Self { inner: CommonText::new(indent_size) }
    }
}

impl Format for Text {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        self.inner.write_str(input, target)
    }
}

/// Nesting scope: advances indentation level every entering, except for the
/// first.
#[derive(Debug, Clone, Copy)]
pub struct Nest;

impl Scope for Nest {
    type Format = Text;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        common_text::Nest.enter(&mut format.inner, |inner| {
            let mut copy = Text { inner: *inner };
            let output = consumer(&mut copy);
            *inner = copy.inner;
            output
        })
    }
}
