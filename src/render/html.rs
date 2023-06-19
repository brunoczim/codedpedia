//! This module provides utilities about the HTML rendering format.

use super::Format;
use std::fmt;

/// HTML5 rendering format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Html;

impl Format for Html {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        target.write_str(input)
    }
}

#[cfg(test)]
pub mod test {
    use scraper::Html;

    pub fn validate_html_fragment(fragment: &str) -> Result<(), Vec<String>> {
        let output = Html::parse_fragment(fragment);
        if output.errors.len() == 0 {
            Ok(())
        } else {
            Err(output.errors.into_iter().map(String::from).collect())
        }
    }

    pub fn validate_html_document(fragment: &str) -> Result<(), Vec<String>> {
        let output = Html::parse_document(fragment);
        if output.errors.len() == 0 {
            Ok(())
        } else {
            Err(output.errors.into_iter().map(String::from).collect())
        }
    }
}
