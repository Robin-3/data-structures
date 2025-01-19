use exceptions::Exceptions;
use std::fmt::{Debug, Formatter, Result as fmtResult};

#[derive(Clone)]
pub struct DynamicArray<T: Clone> {
    array: Box<[Option<T>]>,
    len: usize,
    capacity: usize,
    current: usize,
}

impl<T: Clone> DynamicArray<T> {
    /// Crea un nuevo arreglo dinámico vacío con la capacidad especificada.
    ///
    /// # Parámetros
    /// - `capacity`: La capacidad inicial del arreglo dinámico. Define el número máximo de elementos que puede contener antes de necesitar redimensionarse.
    ///
    /// # Retornos
    /// - Devuelve una nueva instancia de `DynamicArray` con la capacidad especificada y sin elementos iniciales.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let array: DynamicArray<i32> = DynamicArray::new(5);
    ///
    /// // El arreglo tiene capacidad para 5 elementos, pero está vacío.
    /// assert_eq!(array.capacity(), 5);
    /// assert_eq!(array.len(), 0);
    /// assert!(array.is_empty());
    /// ```
    ///
    /// # Notas
    /// - El arreglo dinámico se inicializa con `None` en cada posición, lo que ocupa espacio en memoria según la capacidad especificada.
    /// - Para agregar elementos, utiliza métodos como `push` o `unshift`.
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
            current: 0,
        }
    }

    /// Crea un nuevo arreglo dinámico con una capacidad especificada y elementos iniciales.
    ///
    /// # Parámetros
    /// - `capacity`: La capacidad inicial del arreglo dinámico. Define el número máximo de elementos que puede contener antes de necesitar redimensionarse.
    /// - `values`: Un slice de valores que se utilizarán para inicializar el arreglo.
    ///
    /// # Retornos
    /// - Devuelve una nueva instancia de `DynamicArray` inicializada con los valores proporcionados.
    ///
    /// # Comportamiento
    /// - Si la longitud de `values` es menor que `capacity`, los valores restantes del arreglo se inicializan como `None`.
    /// - Si la longitud de `values` es mayor o igual a `capacity`, solo se toman los primeros `capacity` elementos del slice.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let array = DynamicArray::with_values(5, &[1, 2, 3]);
    ///
    /// // El arreglo tiene capacidad para 5 elementos, pero solo 3 están inicializados.
    /// assert_eq!(array.capacity(), 5);
    /// assert_eq!(array.len(), 3);
    /// assert_eq!(array.get(0), Ok(&1));
    /// assert_eq!(array.get(1), Ok(&2));
    /// assert_eq!(array.get(2), Ok(&3));
    /// assert!(array.get(3).is_err()); // Índices fuera de los valores iniciales retornan error.
    ///
    /// // Si se excede la capacidad, solo se toman los primeros elementos.
    /// let array = DynamicArray::with_values(2, &[10, 20, 30]);
    /// assert_eq!(array.len(), array.capacity());
    /// assert_eq!(array.len(), 2);
    /// assert_eq!(array.get(1), Ok(&20));
    /// assert!(array.get(2).is_err());
    /// ```
    ///
    /// # Notas
    /// - El arreglo dinámico reserva espacio en memoria para la capacidad especificada, pero su longitud inicial (`len`) dependerá de los valores proporcionados.
    /// - Para agregar más elementos después de la creación, utiliza métodos como `push` o `unshift`.
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
            current: 0,
        }
    }

    /// Obtiene una referencia inmutable al elemento en el índice especificado.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea obtener. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(&T)`: Si el índice es válido y el elemento está presente, devuelve una referencia inmutable al elemento.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera de los límites o no hay un valor en esa posición.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// # use exceptions::Exceptions;
    /// let array = DynamicArray::with_values(4, &[1, 2, 3]);
    /// assert_eq!(array.get(1), Ok(&2));
    /// assert!(array.get(4).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará `Exceptions::IndexOutOfBounds` si:
    /// - `index` es mayor o igual a `self.len`.
    /// - No hay un valor presente en el índice especificado.
    pub fn get(&self, index: usize) -> Result<&T, Exceptions> {
        match (index, self.array.get(index)) {
            (i, _) if i >= self.len => Err(Exceptions::IndexOutOfBounds),
            (_, None) => Err(Exceptions::IndexOutOfBounds),
            (_, Some(value)) => value.as_ref().ok_or(Exceptions::IndexOutOfBounds),
        }
    }

    /// Obtiene una referencia mutable al elemento en el índice especificado.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea obtener. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(&mut T)`: Si el índice es válido y el elemento está presente, devuelve una referencia mutable al elemento.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera de los límites o no hay un valor en esa posición.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// # use exceptions::Exceptions;
    /// let mut array = DynamicArray::with_values(3, &[1, 2, 3]);
    /// if let Ok(value) = array.get_mut(1) {
    ///     *value = 42;
    /// }
    /// assert_eq!(array.get(1), Ok(&42));
    /// ```
    ///
    /// # Errors
    /// Este método retornará `Exceptions::IndexOutOfBounds` si:
    /// - `index` es mayor o igual a `self.len`.
    /// - No hay un valor presente en el índice especificado.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, Exceptions> {
        match (index, self.array.get_mut(index)) {
            (i, _) if i >= self.len => Err(Exceptions::IndexOutOfBounds),
            (_, None) => Err(Exceptions::IndexOutOfBounds),
            (_, Some(value)) => value.as_mut().ok_or(Exceptions::IndexOutOfBounds),
        }
    }

    /// Establece un valor en el índice especificado del arreglo dinámico.
    ///
    /// # Parámetros
    /// - `index`: El índice en el que se desea establecer el valor. Debe estar en el rango `0..self.len`.
    /// - `value`: El valor que se desea asignar en el índice especificado.
    ///
    /// # Retornos
    /// - `Ok(())`: Si el índice es válido y se establece el valor correctamente.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera de los límites de la longitud actual del arreglo.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// # use exceptions::Exceptions;
    /// let mut array = DynamicArray::with_values(3, &[1, 2, 3]);
    /// assert_eq!(array.set(1, 42), Ok(()));
    /// assert_eq!(array.get(1), Ok(&42));
    ///
    /// // Intentar establecer un valor fuera de los límites retorna un error.
    /// assert!(array.set(5, 10).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará `Exceptions::IndexOutOfBounds` si:
    /// - `index` es mayor o igual a `self.len`.
    ///
    /// # Notas
    /// Esta función no modifica la capacidad del arreglo. Para añadir nuevos valores
    /// fuera del rango actual, utiliza métodos como `unshift` o `push`.
    pub fn set(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        self.array[index] = Some(value);
        Ok(())
    }

    /// Inserta un elemento al inicio del arreglo dinámico, desplazando los elementos existentes hacia la derecha.
    ///
    /// # Parámetros
    /// - `value`: El valor que se desea insertar al inicio del arreglo.
    ///
    /// # Comportamiento
    /// - Si la capacidad actual del arreglo es `0`, se redimensiona automáticamente a `4`.
    /// - Si la longitud actual (`len`) es igual a la capacidad (`capacity`), el arreglo se redimensiona automáticamente, duplicando su capacidad antes de la inserción.
    /// - Los elementos existentes se desplazan una posición hacia la derecha para dejar espacio al nuevo elemento.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::with_values(3, &[1, 2, 3]);
    ///
    /// assert_eq!(array.len(), array.capacity());
    ///
    /// // Inserta el valor 0 al inicio.
    /// array.unshift(0);
    ///
    /// // El arreglo ahora es [0, 1, 2, 3].
    /// assert_eq!(array.get(0), Ok(&0));
    /// assert_eq!(array.get(1), Ok(&1));
    /// assert_eq!(array.len(), 4);
    /// assert_ne!(array.len(), array.capacity());
    ///
    /// // Inserta un valor en un arreglo vacío.
    /// let mut empty_array = DynamicArray::new(0);
    /// assert_eq!(empty_array.capacity(), 0);
    /// empty_array.unshift(42);
    /// assert_eq!(empty_array.capacity(), 4);
    /// assert_eq!(empty_array.len(), 1);
    /// assert_eq!(empty_array.get(0), Ok(&42));
    /// ```
    ///
    /// # Notas
    /// - Este método puede modificar la capacidad del arreglo dinámico si no hay espacio suficiente para el nuevo elemento.
    /// - El desplazamiento de elementos tiene un costo proporcional a la longitud actual del arreglo (`O(n)`), por lo que es menos eficiente que agregar al final (`push`).
    pub fn unshift(&mut self, value: T) {
        if self.capacity == 0 {
            self.resize(4);
        } else if self.len == self.capacity {
            self.resize(self.capacity * 2);
        }

        let arr = &self.array.clone();
        for i in (1..=self.len).rev() {
            self.array[i].clone_from(&arr[i - 1]);
        }
        self.array[0] = Some(value);
        self.len += 1;
    }

    /// Agrega un elemento al final del arreglo dinámico.
    ///
    /// # Parámetros
    /// - `value`: El valor que se desea agregar al final del arreglo.
    ///
    /// # Comportamiento
    /// - Si la capacidad actual del arreglo es `0`, se redimensiona automáticamente a `4`.
    /// - Si la longitud actual (`len`) es igual a la capacidad (`capacity`), el arreglo se redimensiona automáticamente, duplicando su capacidad antes de agregar el nuevo elemento.
    /// - El nuevo elemento se almacena al final del arreglo, incrementando su longitud en `1`.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::new(0);
    ///
    /// // Agregar elementos al arreglo.
    /// array.push(10);
    /// array.push(20);
    ///
    /// // El arreglo ahora contiene [10, 20].
    /// assert_eq!(array.len(), 2);
    /// assert_eq!(array.get(0), Ok(&10));
    /// assert_eq!(array.get(1), Ok(&20));
    ///
    /// // Agregar más elementos hace que la capacidad crezca automáticamente.
    /// for i in 0..10 {
    ///     array.push(i);
    /// }
    /// assert_eq!(array.len(), 12); // La longitud crece con los nuevos elementos.
    /// assert!(array.capacity() >= array.len());
    /// ```
    ///
    /// # Notas
    /// - Este método puede modificar la capacidad del arreglo dinámico si no hay espacio suficiente para el nuevo elemento.
    /// - El costo de redimensionar (si es necesario) es amortizado, lo que lo hace eficiente para agregar múltiples elementos consecutivos.
    pub fn push(&mut self, value: T) {
        if self.capacity == 0 {
            self.resize(4);
        } else if self.len == self.capacity {
            self.resize(self.capacity * 2);
        }
        self.array[self.len] = Some(value);
        self.len += 1;
    }

    /// Inserta un valor en el índice especificado del arreglo dinámico, desplazando los elementos existentes.
    ///
    /// # Parámetros
    /// - `index`: El índice en el que se desea insertar el valor. Debe estar en el rango `0..self.len`.
    /// - `value`: El valor que se desea insertar.
    ///
    /// # Retornos
    /// - `Ok(())`: Si el índice es válido y el valor se inserta correctamente.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera de los límites de la longitud actual del arreglo.
    ///
    /// # Comportamiento
    /// - Si la longitud actual (`len`) es igual a la capacidad (`capacity`), el arreglo dinámico se redimensiona automáticamente para duplicar su capacidad antes de insertar el valor.
    /// - Los elementos desde el índice especificado hasta el final se desplazan una posición hacia la derecha.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// # use exceptions::Exceptions;
    /// let mut array = DynamicArray::with_values(3, &[1, 2, 3]);
    ///
    /// // Inserta el valor 42 en el índice 1.
    /// assert_eq!(array.insert(1, 42), Ok(()));
    ///
    /// // El arreglo ahora es [1, 42, 2, 3].
    /// assert_eq!(array.get(1), Ok(&42));
    /// assert_eq!(array.get(2), Ok(&2));
    ///
    /// // Intentar insertar fuera de los límites retorna un error.
    /// assert!(array.insert(5, 10).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará `Exceptions::IndexOutOfBounds` si:
    /// - `index` es mayor o igual a `self.len`.
    ///
    /// # Notas
    /// - Este método puede modificar la capacidad del arreglo dinámico si no hay espacio suficiente para insertar un nuevo elemento.
    /// - Inserciones repetidas pueden tener un costo significativo si se requiere redimensionar el arreglo frecuentemente.
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

    /// Elimina el elemento en el índice especificado del arreglo dinámico y devuelve su valor.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea eliminar. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(T)`: Si el índice es válido, devuelve el valor del elemento eliminado.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera de los límites del arreglo.
    ///
    /// # Comportamiento
    /// - Los elementos posteriores al índice especificado se desplazan una posición hacia la izquierda para llenar el espacio vacío.
    /// - Si, después de la eliminación, la longitud del arreglo es menor que la mitad de su capacidad y la capacidad es mayor que `1`, el arreglo se redimensiona automáticamente para reducir su capacidad a la mitad.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// # use exceptions::Exceptions;
    /// let mut array = DynamicArray::with_values(10, &[1, 2, 3]);
    ///
    /// // Eliminar el elemento en el índice 1.
    /// let removed = array.remove(1);
    /// assert_eq!(removed, Ok(2)); // El valor eliminado es 2.
    /// assert_eq!(array.len(), 2); // La longitud se reduce.
    /// assert_eq!(array.get(1), Ok(&3)); // Los elementos se desplazan.
    /// assert!(array.capacity() < 10);
    ///
    /// // Intentar eliminar fuera de los límites retorna un error.
    /// assert!(array.remove(5).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará `Exceptions::IndexOutOfBounds` si:
    /// - `index` es mayor o igual a `self.len`.
    ///
    /// # Notas
    /// - Este método puede modificar la capacidad del arreglo dinámico si, después de la eliminación, su longitud es menor que la mitad de su capacidad.
    /// - Si necesitas eliminar elementos sin redimensionar automáticamente, deberías implementar un método alternativo.
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

    /// Devuelve la cantidad de elementos almacenados actualmente en el arreglo dinámico.
    ///
    /// # Retornos
    /// - `usize`: El número de elementos actualmente presentes en el arreglo.
    ///
    /// # Comportamiento
    /// - La longitud (`len`) representa el número de elementos válidos almacenados en el arreglo, no la capacidad total.
    /// - Los espacios vacíos (inicializados como `None`) no se cuentan como parte de la longitud.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::with_values(5, &[1, 2, 3]);
    ///
    /// // El arreglo tiene 3 elementos inicializados.
    /// assert_eq!(array.len(), 3);
    ///
    /// // Después de agregar un elemento, la longitud aumenta.
    /// array.push(4);
    /// assert_eq!(array.len(), 4);
    ///
    /// // Eliminar un elemento reduce la longitud.
    /// array.remove(0).unwrap();
    /// assert_eq!(array.len(), 3);
    /// ```
    ///
    /// # Notas
    /// - La longitud no debe confundirse con la capacidad, que define el número máximo de elementos que el arreglo puede almacenar sin redimensionarse.
    /// - Para verificar si el arreglo está vacío, utiliza el método `is_empty`.
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Devuelve la capacidad actual del arreglo dinámico.
    ///
    /// # Retornos
    /// - `usize`: El número máximo de elementos que el arreglo puede almacenar sin redimensionarse.
    ///
    /// # Comportamiento
    /// - La capacidad define el tamaño del espacio reservado en memoria para los elementos del arreglo.
    /// - Es independiente de la longitud actual (`len`) y puede ser mayor si se ha reservado espacio adicional.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::new(4);
    ///
    /// // La capacidad inicial es 4.
    /// assert_eq!(array.capacity(), 4);
    ///
    /// // Después de agregar elementos, la capacidad puede crecer.
    /// for i in 0..5 {
    ///     array.push(i);
    /// }
    /// assert!(array.capacity() > 4); // La capacidad se redimensiona automáticamente.
    /// ```
    ///
    /// # Notas
    /// - La capacidad inicial puede configurarse al crear el arreglo mediante `new` o `with_values`.
    /// - Si la longitud (`len`) del arreglo alcanza la capacidad, esta se redimensiona automáticamente, generalmente duplicándose.
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Verifica si el arreglo dinámico está vacío.
    ///
    /// # Retornos
    /// - `true`: Si la longitud actual del arreglo (`len`) es `0`.
    /// - `false`: Si el arreglo contiene uno o más elementos.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::new(4);
    ///
    /// // Un arreglo recién creado está vacío.
    /// assert!(array.is_empty());
    ///
    /// // Después de agregar un elemento, ya no está vacío.
    /// array.push(10);
    /// assert!(!array.is_empty());
    ///
    /// // Al eliminar todos los elementos, vuelve a estar vacío.
    /// array.remove(0).unwrap();
    /// assert!(array.is_empty());
    /// ```
    ///
    /// # Notas
    /// - Este método es equivalente a comprobar si `len() == 0`.
    /// - Útil para validar condiciones antes de realizar operaciones en el arreglo.
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Redimensiona la capacidad del arreglo dinámico.
    ///
    /// # Parámetros
    /// - `new_capacity`: La nueva capacidad del arreglo dinámico. Define el número máximo de elementos que puede contener después de redimensionarse.
    ///
    /// # Comportamiento
    /// - Si `new_capacity` es mayor que la capacidad actual, se crea un nuevo arreglo con la capacidad especificada y los elementos existentes se copian en él.
    /// - Si `new_capacity` es menor que la capacidad actual, se truncan los elementos que excedan la nueva capacidad.
    ///
    /// # Ejemplo
    /// ```
    /// # use array::DynamicArray;
    /// let mut array = DynamicArray::with_values(3, &[1, 2, 3]);
    ///
    /// // Redimensiona para aumentar la capacidad.
    /// array.resize(5);
    /// assert_eq!(array.len(), 3);
    /// assert_eq!(array.capacity(), 5);
    /// assert!(array.get(4).is_err()); // Las nuevas posiciones están vacías.
    ///
    /// // Redimensiona para reducir la capacidad.
    /// array.resize(2);
    /// assert_eq!(array.len(), array.capacity());
    /// assert_eq!(array.capacity(), 2);
    /// assert!(array.get(2).is_err());
    ///
    /// // Redimensionar a capacidad 0.
    /// array.resize(0);
    /// assert_eq!(array.len(), 0);
    /// ```
    ///
    /// # Notas
    /// - Redimensionar a una capacidad menor que la longitud actual (`len`) eliminará elementos desde el final hasta ajustarse a la nueva capacidad.
    pub fn resize(&mut self, new_capacity: usize) {
        let mut new_array: Box<[Option<T>]> = vec![None; new_capacity].into_boxed_slice();

        for (index, value) in self.array.iter().enumerate() {
            if let Some(v) = new_array.get_mut(index) {
                v.clone_from(value);
            }
        }

        self.array = new_array;
        self.capacity = new_capacity;
        if self.len > self.capacity {
            self.len = self.capacity;
        }
    }
}

impl<T: Clone> Iterator for DynamicArray<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            self.current += 1;
            self.array[self.current - 1].clone()
        } else {
            None
        }
    }
}

impl<T: Clone + Debug> Debug for DynamicArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "[")?;
        for (index, value) in self.clone().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value:?}")?;
        }
        write!(f, "]")
    }
}
