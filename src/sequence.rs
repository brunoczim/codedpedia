pub trait Sequence {
    type Item;
    type IterRef<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter<'a>(&'a self) -> Self::IterRef<'a>;
}

impl<S, T> Sequence for S
where
    for<'a> &'a S: IntoIterator<Item = &'a T>,
{
    type Item = T;
    type IterRef<'a> = <&'a S as IntoIterator>::IntoIter
    where
        S: 'a;

    fn iter<'a>(&'a self) -> Self::IterRef<'a> {
        self.into_iter()
    }
}
