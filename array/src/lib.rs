pub fn static_array() {
    println!("Array estático");
    // 1. Initialization
    let mut planets: [Option<&str>; 6] = [None; 6];
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Assigning values
    planets[0] = Some("Venus");
    planets[1] = Some("Plutón");
    planets[2] = Some("Tierra");
    planets[3] = Some("Jupiter");
    println!("  2. Asignación de valores:\n    {planets:?}");
    // 3. Insertion at the beginning
    // Shift elements right
    for i in (1..planets.len()).rev() {
        planets[i] = planets[i - 1];
    }
    // Assing a new value
    planets[0] = Some("Mercurio");
    println!("  3. Insertar al inicio:\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 4;
    for i in ((position + 1)..planets.len()).rev() {
        planets[i] = planets[i - 1];
    }
    planets[position] = Some("Marte");
    println!("  4. Insertar en una posición arbitraria (indice: {position}):\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet: Option<&str> = planets[position];
    // Shift elements left
    for i in position..(planets.len() - 1) {
        planets[i] = planets[i + 1];
    }
    planets[planets.len() - 1] = None;
    println!("  5. Eliminar de una posición arbitraria: (indice: {position}, planeta: {planet:?})\n    {planets:?}");
}

pub fn dynamic_array() {
    println!("Array dinámico");
    // 1. Initialization
    let mut planets: Vec<&str> = Vec::new();
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Insertion at the end
    planets.push("Venus");
    planets.push("Plutón");
    planets.push("Tierra");
    planets.push("Jupiter");
    println!("  2. Insertar al final:\n    {planets:?}");
    // 3. Insertion at the beginning
    let position: usize = 0;
    planets.insert(position, "Mercurio");
    println!("  3. Insertar al inicio: (indice: {position})\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 4;
    planets.insert(position, "Marte");
    println!("  4. Insertar en una posición arbitraria  (indice: {position}):\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet: &str = planets.remove(position);
    println!("  5. Eliminar de una posición arbitraria (indice: {position}, planeta: {planet:?})\n    {planets:?}");
}

use dynamic_array_implementation::DynamicArray;
use exceptions::Exceptions;

pub fn da_implementation() -> Result<(), Exceptions> {
    println!("Implementación de un array dinámico");
    // 1. Initialization
    let planets: DynamicArray<&str> = DynamicArray::new(5);
    println!("  1. Inicialización:\n    {planets:?}");
    // 2. Initializacion with values
    let mut planets: DynamicArray<&str> =
        DynamicArray::with_values(5, &["Venus", "Plutón", "Tierra", "Marte"]);
    println!("  2.  Inicialización con valores\n    {planets:?}");
    // 3. Insertion at the ending
    planets.add("Jupiter");
    println!("  3. Insertar al final\n    {planets:?}");
    // 4. Insertion at an arbitrary position
    let position: usize = 0;
    planets.insert(position, "Mercurio")?;
    println!("  4.  Insertar en una posición arbitraria: (index: {position})\n    {planets:?}");
    // 5. Deletion at an arbitrary position
    let position: usize = 2;
    let planet: Option<&str> = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    let position: usize = 0;
    let planet: Option<&str> = planets.remove(position)?;
    println!("  5. Eliminar de una posición arbitraria: (index: {position}, planeta: {planet:?})\n    {planets:?}");
    Ok(())
}

pub mod dynamic_array_implementation {
    use exceptions::Exceptions;

    // A dynamic array implementation based on a static array
    #[derive(Debug)]
    pub struct DynamicArray<T: Copy> {
        // Underlying static array
        array: Vec<Option<T>>,
        // Current number of elements in the array
        size: usize,
        // Maximum number of elements the array can hold without resizing
        capacity: usize,
    }

    impl<T: Copy> DynamicArray<T> {
        // Constructs a new dynamic array with the given initial capacity
        pub fn new(capacity: usize) -> Self {
            let array: Vec<Option<T>> = vec![None; capacity];

            Self {
                array,
                size: 0,
                capacity,
            }
        }

        // Constructs a new dynamic array with the given initial capacity, and values
        pub fn with_values(capacity: usize, values: &[T]) -> Self {
            let mut array: Vec<Option<T>> = vec![None; capacity];

            for (index, value) in values.iter().enumerate() {
                array[index] = Some(value.to_owned());
            }

            let size: usize = if values.len() < capacity {
                values.len()
            } else {
                capacity
            };

            Self {
                array,
                size,
                capacity,
            }
        }

        // Returns the value at the given index in the dynamic array
        pub fn get(&self, index: usize) -> Result<Option<T>, Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            Ok(self.array[index])
        }

        // Sets the value at the given index in the dynamic array to the given value
        pub fn set(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            self.array[index] = Some(value);
            Ok(())
        }

        // Adds a new value to the end of the dynamic array
        pub fn add(&mut self, value: T) {
            if self.size == self.capacity {
                self.resize(self.capacity * 2);
            }
            self.array[self.size] = Some(value);
            self.size += 1;
        }

        // Adds a new value at the specified index in the dynamic array
        pub fn insert(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            if self.size == self.capacity {
                self.resize(self.capacity * 2);
            }
            for i in ((index + 1)..=self.size).rev() {
                self.array[i] = self.array[i - 1];
            }
            self.array[index] = Some(value);
            self.size += 1;
            Ok(())
        }

        // Remove the value at the given index from the dynamic array
        pub fn remove(&mut self, index: usize) -> Result<Option<T>, Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            let value = self.array[index];
            for i in index..(self.size - 1) {
                self.array[i] = self.array[i + 1];
            }
            self.array[self.size - 1] = None;
            self.size -= 1;
            if self.size < self.capacity / 2 && self.capacity > 1 {
                self.resize(self.capacity / 2);
            }
            Ok(value)
        }

        // Returns the current size of the dynamic array
        pub const fn size(&self) -> usize {
            self.size
        }

        // Adjusts the capacity of the dynamic array by creating a new array with the given capacity, copying the elements from the old array to the new array, and updating the capacity and reference to the old array
        fn resize(&mut self, new_capacity: usize) {
            let mut new_array: Vec<Option<T>> = vec![None; new_capacity];

            for (index, value) in self.array.iter().enumerate() {
                if let Some(v) = new_array.get_mut(index) {
                    *v = *value;
                }
            }

            self.array = new_array;
            self.capacity = new_capacity;
        }
    }
}
