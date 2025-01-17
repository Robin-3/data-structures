use exceptions::Exceptions;

// A dynamic array implementation based on a static array
#[derive(Debug)]
pub struct DynamicArray<T: Clone> {
    // Underlying static array
    array: Box<[Option<T>]>,
    // Current number of elements in the array
    len: usize,
    // Maximum number of elements the array can hold without resizing
    capacity: usize,
}

impl<T: Clone> DynamicArray<T> {
    // Constructs a new dynamic array with the given initial capacity
    pub fn new(capacity: usize) -> Self {
        let mut vec: Vec<Option<T>> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            vec.push(None);
        }
        let array: Box<[Option<T>]> = vec.into_boxed_slice();

        Self {
            array,
            len: 0,
            capacity,
        }
    }

    // Constructs a new dynamic array with the given initial capacity, and values
    pub fn with_values(capacity: usize, values: &[T]) -> Self {
        let size: usize = if values.len() < capacity {
            values.len()
        } else {
            capacity
        };

        let mut vec: Vec<Option<T>> = Vec::with_capacity(capacity);
        for index in 0..capacity {
            match values.get(index) {
                Some(value) => vec.push(Some(value.to_owned())),
                None => vec.push(None),
            }
        }
        let array: Box<[Option<T>]> = vec.into_boxed_slice();

        Self {
            array,
            len: size,
            capacity,
        }
    }

    // Returns the value at the given index in the dynamic array
    pub fn get(&self, index: usize) -> Result<&T, Exceptions> {
        match (index, self.array.get(index)) {
            (i, _) if i >= self.len => Err(Exceptions::IndexOutOfBounds),
            (_, None) => Err(Exceptions::IndexOutOfBounds),
            (_, Some(value)) => value.as_ref().ok_or(Exceptions::IndexOutOfBounds),
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, Exceptions> {
        match (index, self.array.get_mut(index)) {
            (i, _) if i >= self.len => Err(Exceptions::IndexOutOfBounds),
            (_, None) => Err(Exceptions::IndexOutOfBounds),
            (_, Some(value)) => value.as_mut().ok_or(Exceptions::IndexOutOfBounds),
        }
    }

    // Sets the value at the given index in the dynamic array to the given value
    pub fn set(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        self.array[index] = Some(value);
        Ok(())
    }
    // Adds a new value to the strat of the dynamic array
    pub fn add(&mut self, value: T) -> Result<(), Exceptions> {
        self.insert(0, value)
    }

    // Adds a new value to the end of the dynamic array
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.resize(self.capacity * 2);
        }
        self.array[self.len] = Some(value);
        self.len += 1;
    }

    // Adds a new value at the specified index in the dynamic array
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        if self.len == self.capacity {
            self.resize(self.capacity * 2);
        }

        let arr = &self.array.clone();
        for i in ((index + 1)..=self.len).rev() {
            self.array[i].clone_from(&arr[i - 1]);
        }
        self.array[index] = Some(value);
        self.len += 1;
        Ok(())
    }

    // Remove the value at the given index from the dynamic array
    pub fn remove(&mut self, index: usize) -> Result<T, Exceptions> {
        let value = match (index, self.array.get(index)) {
            (i, _) if i >= self.len => return Err(Exceptions::IndexOutOfBounds),
            (_, None) => return Err(Exceptions::IndexOutOfBounds),
            (_, Some(value)) => match value {
                Some(v) => v.clone(),
                None => return Err(Exceptions::IndexOutOfBounds),
            },
        };

        let slice = &self.array.clone()[(index + 1)..self.len];
        for (i, v) in slice.iter().enumerate() {
            self.array[index + i].clone_from(v);
        }
        self.array[self.len - 1] = None;
        self.len -= 1;
        if self.len < self.capacity / 2 && self.capacity > 1 {
            self.resize(self.capacity / 2);
        }
        Ok(value)
    }

    // Returns the current size of the dynamic array
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Adjusts the capacity of the dynamic array by creating a new array with the given capacity, copying the elements from the old array to the new array, and updating the capacity and reference to the old array
    fn resize(&mut self, new_capacity: usize) {
        let mut new_array: Box<[Option<T>]> = vec![None; new_capacity].into_boxed_slice();

        for (index, value) in self.array.iter().enumerate() {
            if let Some(v) = new_array.get_mut(index) {
                v.clone_from(value);
            }
        }

        self.array = new_array;
        self.capacity = new_capacity;
    }
}
