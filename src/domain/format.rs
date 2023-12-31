use std::fmt;

pub trait Format {
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result;
}

impl<'this, W> Format for &'this mut W
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}

impl<W> Format for Box<W>
where
    W: Format + ?Sized,
{
    fn write_str(
        &mut self,
        input: &str,
        target: &mut dyn fmt::Write,
    ) -> fmt::Result {
        (**self).write_str(input, target)
    }
}
