#[derive(Debug, Clone)]
pub struct Entry<T: Clone> {
    // index: usize, // Si quisiera recuperar todo el orden de creación
    key: String,
    value: T,
}

impl<T: Clone> Entry<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn compare_key(&self, key: &String) -> bool {
        &self.key == key
    }

    pub fn get_entry(&self) -> (&String, &T) {
        (&self.key, &self.value)
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}
