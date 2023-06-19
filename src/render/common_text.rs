use super::{Format, Scope};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NewlineState {
    Unfinished,
    Starting { needs_flush: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommonText {
    newline_state: NewlineState,
    level: u32,
    indent_size: u32,
}

impl Default for CommonText {
    fn default() -> Self {
        Self::new(4)
    }
}

impl CommonText {
    pub fn new(indent_size: u32) -> Self {
        Self {
            newline_state: NewlineState::Starting { needs_flush: false },
            level: 0,
            indent_size,
        }
    }
}

impl Format for CommonText {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        for line in input.split_inclusive('\n') {
            match self.newline_state {
                NewlineState::Unfinished => {
                    target.write_str(line)?;
                    if line.ends_with('\n') {
                        self.newline_state =
                            NewlineState::Starting { needs_flush: false };
                    }
                },

                NewlineState::Starting { needs_flush } => {
                    if needs_flush {
                        target.write_str("\n")?;
                    }
                    if line == "\n" {
                        self.newline_state =
                            NewlineState::Starting { needs_flush: true };
                    } else {
                        for _ in
                            0 .. self.indent_size * self.level.saturating_sub(1)
                        {
                            target.write_str(" ")?;
                        }
                        target.write_str(line)?;
                        self.newline_state = if line.ends_with('\n') {
                            NewlineState::Starting { needs_flush: false }
                        } else {
                            NewlineState::Unfinished
                        };
                    }
                },
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Nest;

impl Scope for Nest {
    type Format = CommonText;

    fn enter<F, T>(&self, format: &mut Self::Format, consumer: F) -> T
    where
        F: FnOnce(&mut Self::Format) -> T,
    {
        format.level += 1;
        let output = consumer(format);
        format.level -= 1;
        output
    }
}

#[cfg(test)]
mod test {
    use super::{CommonText, Nest};
    use crate::render::Renderer;
    use std::fmt::Write;

    #[test]
    fn newlines() {
        let mut output = String::new();
        let mut format = CommonText::default();
        let mut renderer = Renderer::new(&mut format, &mut output);
        write!(renderer, "abcd").unwrap();
        write!(renderer, "  ").unwrap();
        write!(renderer, "efg\n\n").unwrap();
        write!(renderer, " ").unwrap();
        write!(renderer, "123\n\n").unwrap();

        assert_eq!(output, "abcd  efg\n\n 123\n");
    }

    #[test]
    fn nest() {
        let mut output = String::new();
        let mut format = CommonText::default();
        let mut renderer = Renderer::new(&mut format, &mut output);
        write!(renderer, "abcdefg\n").unwrap();

        renderer
            .scoped(Nest, |renderer| write!(renderer, "123\n4567\nh\n"))
            .unwrap();

        write!(renderer, "ijk\n").unwrap();

        renderer
            .scoped(Nest, |renderer| {
                write!(renderer, "a1\nb\n")?;
                renderer.scoped(Nest, |renderer| {
                    write!(renderer, "c\nd2\n")?;
                    renderer.scoped(Nest, |renderer| {
                        write!(renderer, "idk\nyeah\n")
                    })
                })?;
                write!(renderer, "eee\n")
            })
            .unwrap();

        assert_eq!(
            output,
            "abcdefg\n123\n4567\nh\nijk\na1\nb\n    c\n    d2\n        idk\n        yeah\neee\n",
        );
    }
}
