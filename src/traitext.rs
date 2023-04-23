pub trait ExpectOnlyOneExt<T>
where
    Self: Iterator<Item = T>,
{
    fn unwrap_only_one(self) -> T;
    fn expect_only_one(self, message: &str) -> T;
}

impl<T, I> ExpectOnlyOneExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn unwrap_only_one(mut self) -> T {
        let Some(first) = self.next() else {
            panic!("No item in the iterator");
        };

        if self.next().is_some() {
            panic!("Expected only one item in the iterator");
        }

        first
    }

    fn expect_only_one(mut self, message: &str) -> T {
        let Some(first) = self.next() else {
            panic!("No item in the iterator: {}", message);
        };

        if self.next().is_some() {
            panic!("Expected only one item in the iterator: {}", message);
        }

        first
    }
}
