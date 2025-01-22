pub struct SeparateChainingHashTableIterator<'a, T: Clone> {
    entries: Vec<(&'a String, &'a T)>,
    current: usize,
}

impl<'a, T: Clone> SeparateChainingHashTableIterator<'a, T> {
    pub const fn new(entries: Vec<(&'a String, &'a T)>) -> Self {
        Self {
            entries,
            current: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for SeparateChainingHashTableIterator<'a, T> {
    type Item = (&'a String, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.entries.get(self.current) {
            Some(entry) => {
                self.current += 1;
                Some(*entry)
            }
            None => None,
        }
    }
}
