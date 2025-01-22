mod entry;

use entry::Entry;
use exceptions::Exceptions;

#[derive(Debug)]
pub struct SeparateChainingHashTable<T: Clone> {
    buckets: Box<[Vec<Entry<T>>]>,
    entries_len: usize,
}

impl<T: Clone> SeparateChainingHashTable<T> {
    pub fn new(capacity: usize) -> Self {
        let buckets: Box<[Vec<Entry<T>>]> = vec![Vec::new(); capacity].into_boxed_slice();

        Self {
            buckets,
            entries_len: 0,
        }
    }

    pub fn get<S: Into<String>>(&self, key: S) -> Result<&T, Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        for entry in self.buckets[index].iter() {
            if entry.compare_key(&key) {
                return Ok(entry.get());
            }
        }
        Err(Exceptions::KeyNotInitialized)
    }

    pub fn get_mut<S: Into<String>>(&mut self, key: S) -> Result<&mut T, Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        for entry in self.buckets[index].iter_mut() {
            if entry.compare_key(&key) {
                return Ok(entry.get_mut());
            }
        }
        Err(Exceptions::KeyNotInitialized)
    }

    pub fn set<S: Into<String>>(&mut self, key: S, value: T) -> Result<(), Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        for entry in self.buckets[index].iter_mut() {
            if entry.compare_key(&key) {
                entry.set(value);
                return Ok(());
            }
        }
        Err(Exceptions::KeyNotInitialized)
    }

    pub fn insert<S: Into<String>>(&mut self, key: S, value: T) -> Result<(), Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        let key_exist = self.buckets[index]
            .iter()
            .any(|entry| entry.compare_key(&key));
        if key_exist {
            Err(Exceptions::DuplicateKey)
        } else {
            let entry = Entry::new(key, value);
            self.buckets[index].push(entry);
            self.entries_len += 1;
            Ok(())
        }
    }

    pub fn remove<S: Into<String>>(&mut self, key: S) -> Result<T, Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        let find_entry: Option<usize> = self.buckets[index]
            .iter()
            .position(|entry| entry.compare_key(&key));
        match find_entry {
            Some(index) => {
                let entry: Entry<T> = self.buckets[index].remove(index);
                self.entries_len -= 1;
                Ok(entry.get().to_owned())
            }
            None => Err(Exceptions::KeyNotInitialized),
        }
    }

    pub fn entries_len(&self) -> usize {
        self.entries_len
    }

    pub fn buckets_len(&self) -> usize {
        self.buckets.iter().filter(|entries| !entries.is_empty()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.entries_len == 0
    }

    pub fn hash(value: &String) -> usize {
        let mut h: usize = 0;
        for val in value.as_bytes() {
            h = h.wrapping_add(*val as usize);
        }
        h
    }
}
