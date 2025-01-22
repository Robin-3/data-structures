mod entry;
mod iterator;

use entry::Entry;
use exceptions::Exceptions;
use iterator::SeparateChainingHashTableIterator;
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub struct SeparateChainingHashTable<T: Clone> {
    buckets: Box<[Vec<Entry<T>>]>,
    entries_len: usize,
}

impl<T: Clone> SeparateChainingHashTable<T> {
    #[must_use]
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
        for entry in &self.buckets[index] {
            if entry.compare_key(&key) {
                return Ok(entry.get());
            }
        }
        Err(Exceptions::KeyNotInitialized)
    }

    pub fn get_mut<S: Into<String>>(&mut self, key: S) -> Result<&mut T, Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        for entry in &mut self.buckets[index] {
            if entry.compare_key(&key) {
                return Ok(entry.get_mut());
            }
        }
        Err(Exceptions::KeyNotInitialized)
    }

    pub fn set<S: Into<String>>(&mut self, key: S, value: T) -> Result<(), Exceptions> {
        let key: String = key.into();
        let index = Self::hash(&key) % self.buckets.len();
        for entry in &mut self.buckets[index] {
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

    #[must_use]
    pub fn get_values(&self) -> Vec<&T> {
        let mut values: Vec<&T> = Vec::with_capacity(self.entries_len);
        for entries in &self.buckets {
            if !entries.is_empty() {
                for entry in entries {
                    values.push(entry.get());
                }
            }
        }
        values
    }

    #[must_use]
    pub fn get_keys(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = Vec::with_capacity(self.entries_len);
        for entries in &self.buckets {
            if !entries.is_empty() {
                for entry in entries {
                    keys.push(entry.get_key());
                }
            }
        }
        keys
    }

    #[must_use]
    pub fn get_entries(&self) -> Vec<(&String, &T)> {
        let mut keys_values: Vec<(&String, &T)> = Vec::with_capacity(self.entries_len);
        for entries in &self.buckets {
            if !entries.is_empty() {
                for entry in entries {
                    keys_values.push(entry.get_entry());
                }
            }
        }
        keys_values
    }

    #[must_use]
    pub const fn entries_len(&self) -> usize {
        self.entries_len
    }

    #[must_use]
    pub fn buckets_len(&self) -> usize {
        self.buckets
            .iter()
            .filter(|entries| !entries.is_empty())
            .count()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries_len == 0
    }

    pub fn hash(value: &String) -> usize {
        let mut h: usize = 0;
        for val in value.as_bytes() {
            h = h.wrapping_add(*val as usize);
        }
        h
    }

    pub fn rehashing(&mut self, capacity: usize) {
        let buckets = self.buckets.clone();
        self.buckets = vec![Vec::new(); capacity].into_boxed_slice();
        self.entries_len = 0;
        // let mut keys_values: Vec<(&String, &T)> = Vec::with_capacity(self.entries_len);
        for entries in buckets {
            if !entries.is_empty() {
                for entry in entries {
                    let _ = self.insert(entry.get_key(), entry.get().to_owned());
                }
            }
        }
    }

    #[must_use]
    pub fn iter(&self) -> SeparateChainingHashTableIterator<T> {
        SeparateChainingHashTableIterator::new(self.get_entries())
    }
}

impl<'a, T: Clone> IntoIterator for &'a SeparateChainingHashTable<T> {
    type Item = (&'a String, &'a T);
    type IntoIter = SeparateChainingHashTableIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone + Debug> Debug for SeparateChainingHashTable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{{")?;
        for (index, (key, value)) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key:?}: {value:?}")?;
        }
        write!(f, "}}")
    }
}
