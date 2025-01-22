mod iterator;
mod node;

use exceptions::Exceptions;
use iterator::SinglyLinkedListIterator;
use node::Node;
use std::fmt::{Debug, Formatter, Result as fmtResult};

#[derive(Clone)]
pub struct SinglyLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: Clone> SinglyLinkedList<T> {
    /// Crea una nueva lista enlazada simple vacía.
    ///
    /// # Retornos
    /// - Devuelve una instancia vacía de `SinglyLinkedList` sin nodos y con longitud inicial de `0`.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
    ///
    /// // La lista está vacía.
    /// assert!(list.is_empty());
    /// assert_eq!(list.len(), 0);
    /// ```
    ///
    /// # Notas
    /// - Esta función es una operación de tiempo constante (`O(1)`).
    /// - La lista enlazada creada no contiene nodos iniciales y puede usarse inmediatamente para agregar elementos.
    #[must_use]
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Crea una nueva lista enlazada simple con un único nodo inicializado con el valor proporcionado.
    ///
    /// # Parámetros
    /// - `data`: El valor inicial con el que se creará el primer nodo de la lista.
    ///
    /// # Retornos
    /// - Devuelve una instancia de `SinglyLinkedList` con un único nodo que contiene el valor proporcionado y una longitud inicial de `1`.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let list = SinglyLinkedList::with_data(42);
    ///
    /// // La lista tiene un nodo con el valor 42.
    /// assert_eq!(list.len(), 1);
    /// assert_eq!(list.get(0), Ok(&42));
    /// ```
    ///
    /// # Notas
    /// - Esta función es útil para crear una lista enlazada no vacía con un valor inicial.
    /// - La lista creada puede expandirse añadiendo más nodos con métodos como `push` o `unshift`.
    pub fn with_data(data: T) -> Self {
        let node = Node::new(data);

        Self {
            head: Some(Box::new(node)),
            len: 1,
        }
    }

    /// Obtiene una referencia inmutable al elemento en el índice especificado de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea obtener. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(&T)`: Si el índice es válido, devuelve una referencia inmutable al elemento en ese índice.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera del rango válido de la lista.
    /// - `Err(Exceptions::NoSuchElement)`: Si el índice no corresponde a ningún elemento, lo que puede ocurrir debido a inconsistencias inesperadas.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let list = SinglyLinkedList::with_data(42);
    ///
    /// // Obtener el primer elemento.
    /// assert_eq!(list.get(0), Ok(&42));
    ///
    /// // Intentar acceder a un índice fuera de rango.
    /// assert!(list.get(1).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::IndexOutOfBounds` si `index` es mayor o igual a la longitud de la lista (`len`).
    /// - `Exceptions::NoSuchElement` si ocurre un error inesperado al intentar encontrar el elemento en el índice.
    ///
    /// # Notas
    /// - La operación tiene un costo lineal (`O(n)`), ya que requiere recorrer los nodos de la lista hasta alcanzar el índice solicitado.
    /// - Si necesitas acceso frecuente a los elementos por índice, considera una estructura de datos con mejor rendimiento para este tipo de operación, como un array.
    pub fn get(&self, index: usize) -> Result<&T, Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        let mut pred: &Option<Box<Node<T>>> = &self.head;
        let mut i: usize = 0;
        while let Some(ref node) = pred {
            if i == index {
                return Ok(node.get());
            }
            i += 1;
            pred = node.get_next();
        }
        Err(Exceptions::NoSuchElement(String::from("Element not found")))
    }

    /// Obtiene una referencia mutable al elemento en el índice especificado de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea obtener. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(&mut T)`: Si el índice es válido, devuelve una referencia mutable al elemento en ese índice.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera del rango válido de la lista.
    /// - `Err(Exceptions::NoSuchElement)`: Si el índice no corresponde a ningún elemento, lo que puede ocurrir debido a inconsistencias inesperadas.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::with_data(42);
    ///
    /// // Obtener y modificar el primer elemento.
    /// if let Ok(value) = list.get_mut(0) {
    ///     *value = 100;
    /// }
    /// assert_eq!(list.get(0), Ok(&100));
    ///
    /// // Intentar acceder a un índice fuera de rango.
    /// assert!(list.get_mut(1).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::IndexOutOfBounds` si `index` es mayor o igual a la longitud de la lista (`len`).
    /// - `Exceptions::NoSuchElement` si ocurre un error inesperado al intentar encontrar el elemento en el índice.
    ///
    /// # Notas
    /// - La operación tiene un costo lineal (`O(n)`), ya que requiere recorrer los nodos de la lista hasta alcanzar el índice solicitado.
    /// - Este método permite modificar directamente el valor almacenado en el nodo especificado.
    pub fn get_mut(&mut self, index: usize) -> Result<&mut T, Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        let mut i: usize = 0;
        while let Some(ref mut node) = pred {
            if i == index {
                return Ok(node.get_mut());
            }
            i += 1;
            pred = node.get_next_mut();
        }
        Err(Exceptions::NoSuchElement(String::from("Element not found")))
    }

    /// Establece un nuevo valor en el índice especificado de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea modificar. Debe estar en el rango `0..self.len`.
    /// - `value`: El nuevo valor que se asignará al nodo en el índice especificado.
    ///
    /// # Retornos
    /// - `Ok(())`: Si el índice es válido, el valor se actualiza correctamente.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera del rango válido de la lista.
    /// - `Err(Exceptions::NoSuchElement)`: Si ocurre un error inesperado al intentar encontrar el nodo en el índice.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::with_data(42);
    ///
    /// // Modificar el valor del primer elemento.
    /// assert_eq!(list.set(0, 100), Ok(()));
    /// assert_eq!(list.get(0), Ok(&100));
    ///
    /// // Intentar modificar un índice fuera de rango.
    /// assert!(list.set(1, 200).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::IndexOutOfBounds` si `index` es mayor o igual a la longitud de la lista (`len`).
    /// - `Exceptions::NoSuchElement` si ocurre un error inesperado al intentar encontrar el nodo en el índice.
    ///
    /// # Notas
    /// - La operación tiene un costo lineal (`O(n)`), ya que requiere recorrer los nodos de la lista hasta alcanzar el índice solicitado.
    /// - Este método sobrescribe el valor existente en el nodo especificado con el nuevo valor proporcionado.
    pub fn set(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        let mut i: usize = 0;
        while let Some(ref mut node) = pred {
            if i == index {
                node.set(value);
                return Ok(());
            }
            i += 1;
            pred = node.get_next_mut();
        }
        Err(Exceptions::NoSuchElement(String::from("Element not found")))
    }

    /// Inserta un nuevo elemento al inicio de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `value`: El valor que se desea agregar al inicio de la lista.
    ///
    /// # Comportamiento
    /// - Crea un nuevo nodo con el valor proporcionado y lo coloca como el primer nodo de la lista.
    /// - El nodo previamente ubicado al inicio de la lista pasa a ser el siguiente nodo del nuevo nodo.
    /// - Incrementa la longitud de la lista (`len`) en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::new();
    ///
    /// // Insertar elementos al inicio.
    /// list.unshift(10);
    /// list.unshift(20);
    ///
    /// // La lista ahora contiene [20, 10].
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.get(0), Ok(&20));
    /// assert_eq!(list.get(1), Ok(&10));
    /// ```
    ///
    /// # Notas
    /// - Este método tiene un costo constante (`O(1)`), ya que no requiere recorrer la lista para realizar la inserción.
    /// - Es útil para agregar elementos rápidamente al inicio de la lista.
    pub fn unshift(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);
        node.set_next(self.head.take());
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    /// Inserta un nuevo elemento al final de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `value`: El valor que se desea agregar al final de la lista.
    ///
    /// # Comportamiento
    /// - Si la lista está vacía, el nuevo nodo se convierte en el primer nodo (`head`) de la lista.
    /// - Si la lista ya contiene nodos, recorre la lista hasta encontrar el último nodo y lo enlaza con el nuevo nodo.
    /// - Incrementa la longitud de la lista (`len`) en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::new();
    ///
    /// // Insertar elementos al final.
    /// list.push(10);
    /// list.push(20);
    ///
    /// // La lista ahora contiene [10, 20].
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(list.get(0), Ok(&10));
    /// assert_eq!(list.get(1), Ok(&20));
    /// ```
    ///
    /// # Notas
    /// - Este método tiene un costo lineal (`O(n)`), ya que puede ser necesario recorrer toda la lista para encontrar el último nodo.
    /// - Si necesitas agregar elementos frecuentemente al final de la lista, podrías considerar optimizaciones adicionales, como mantener una referencia al último nodo.
    pub fn push(&mut self, value: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(value)));
            self.len += 1;
            return;
        }
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(ref mut node) = pred {
            if node.get_next().is_none() {
                node.set_next(Some(Box::new(Node::new(value))));
                self.len += 1;
                return;
            }
            pred = node.get_next_mut();
        }
    }

    /// Inserta un nuevo elemento en la lista enlazada simple en un índice específico.
    ///
    /// # Parámetros
    /// - `index`: El índice donde se desea insertar el nuevo elemento. Debe estar en el rango `0..=self.len`.
    /// - `value`: El valor que se desea insertar en el índice especificado.
    ///
    /// # Retornos
    /// - `Ok(())`: Si el índice es válido, el valor se inserta correctamente.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera del rango permitido.
    ///
    /// # Comportamiento
    /// - Si `index` es `0`, el elemento se inserta al inicio de la lista, utilizando internamente el método `unshift`.
    /// - Si `index` está entre `1` y `len - 1`, la lista se recorre hasta el nodo previo al índice, y el nuevo nodo se enlaza en la posición indicada.
    /// - Incrementa la longitud de la lista (`len`) en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::with_data(10);
    ///
    /// // Insertar un elemento en el índice 0 (al inicio).
    /// assert_eq!(list.insert(0, 5), Ok(()));
    /// assert_eq!(list.get(0), Ok(&5));
    ///
    /// // Insertar un elemento en el índice 1.
    /// assert_eq!(list.insert(1, 15), Ok(()));
    /// assert_eq!(list.get(1), Ok(&15));
    ///
    /// // Intentar insertar en un índice fuera de rango.
    /// assert!(list.insert(3, 20).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::IndexOutOfBounds` si `index` es mayor que `self.len`.
    ///
    /// # Notas
    /// - Este método tiene un costo lineal (`O(n)`), ya que requiere recorrer los nodos de la lista hasta el índice especificado.
    /// - Para agregar elementos al inicio o al final, considera usar `unshift` o `push`, ya que son más eficientes.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
        if index >= self.len {
            return Err(Exceptions::IndexOutOfBounds);
        }
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        if index > 0 {
            let mut i: usize = 0;
            while let Some(ref mut node) = pred {
                if i == index - 1 {
                    let mut new_node = Node::new(value);
                    new_node.set_next(node.get_next_mut().take());
                    node.set_next(Some(Box::new(new_node)));
                    self.len += 1;
                    return Ok(());
                }
                i += 1;
                pred = node.get_next_mut();
            }
        } else {
            self.unshift(value);
            return Ok(());
        }
        Err(Exceptions::IndexOutOfBounds)
    }

    /// Elimina y devuelve el primer elemento de la lista enlazada simple.
    ///
    /// # Retornos
    /// - `Ok(T)`: Si la lista no está vacía, devuelve el valor del primer elemento eliminado.
    /// - `Err(Exceptions::NoSuchElement)`: Si la lista está vacía.
    ///
    /// # Comportamiento
    /// - El primer nodo de la lista se elimina y su valor se devuelve.
    /// - El siguiente nodo en la lista se convierte en el nuevo nodo principal (`head`).
    /// - La longitud de la lista (`len`) se reduce en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::with_data(42);
    ///
    /// // Eliminar el primer elemento.
    /// let shifted = list.shift();
    /// assert_eq!(shifted, Ok(42)); // El valor eliminado es 42.
    /// assert!(list.is_empty()); // La lista ahora está vacía.
    ///
    /// // Intentar eliminar de una lista vacía retorna un error.
    /// assert!(list.shift().is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::NoSuchElement` si la lista está vacía.
    ///
    /// # Notas
    /// - Este método tiene un costo constante (`O(1)`), ya que elimina directamente el nodo principal (`head`).
    /// - Es útil para operar sobre listas como colas (`FIFO`), donde los elementos se eliminan del frente.
    pub fn shift(&mut self) -> Result<T, Exceptions> {
        match self.head.take() {
            Some(node) => {
                node.get_next().clone_into(&mut self.head);
                self.len -= 1;
                Ok(node.get().to_owned())
            }
            None => Err(Exceptions::NoSuchElement(String::from("The list is empty"))),
        }
    }

    /// Elimina y devuelve el último elemento de la lista enlazada simple.
    ///
    /// # Retornos
    /// - `Ok(T)`: Si la lista no está vacía, devuelve el valor del último elemento eliminado.
    /// - `Err(Exceptions::NoSuchElement)`: Si la lista está vacía.
    ///
    /// # Comportamiento
    /// - Si la lista contiene un solo nodo, este se elimina, y la lista queda vacía.
    /// - Si la lista tiene múltiples nodos, recorre los nodos hasta el penúltimo, elimina el último nodo y ajusta el enlace del penúltimo para que apunte a `None`.
    /// - La longitud de la lista (`len`) se reduce en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::from(&[42, 100]);
    ///
    /// // Eliminar el último elemento.
    /// let popped = list.pop();
    /// assert_eq!(popped, Ok(100)); // El valor eliminado es 100.
    /// assert_eq!(list.len(), 1); // La lista tiene un nodo restante.
    ///
    /// // Eliminar el último nodo restante.
    /// let popped = list.pop();
    /// assert_eq!(popped, Ok(42));
    /// assert!(list.is_empty()); // La lista ahora está vacía.
    ///
    /// // Intentar eliminar de una lista vacía retorna un error.
    /// assert!(list.pop().is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::NoSuchElement` si la lista está vacía.
    ///
    /// # Notas
    /// - Este método tiene un costo lineal (`O(n)`), ya que requiere recorrer la lista para encontrar el penúltimo nodo si la lista tiene múltiples elementos.
    /// - Es útil para operar sobre listas como pilas (`LIFO`), donde los elementos se eliminan del final.
    pub fn pop(&mut self) -> Result<T, Exceptions> {
        match self.head.take() {
            Some(mut node) => {
                if node.get_next().is_none() {
                    self.head = None;
                    self.len -= 1;
                    Ok(node.get().to_owned())
                } else {
                    let mut list: Self = Self::default();
                    list.push(node.get().to_owned());
                    let mut pred = &mut node;
                    while let Some(ref mut current) = pred.get_next_mut() {
                        if current.get_next().is_none() {
                            let last_node = current;
                            self.head = list.head;
                            self.len -= 1;
                            return Ok(last_node.get().to_owned());
                        }
                        list.push(current.get().to_owned());
                        pred = current;
                    }
                    Err(Exceptions::NoSuchElement(String::from("Element not found")))
                }
            }
            None => Err(Exceptions::NoSuchElement(String::from("The list is empty"))),
        }
    }

    /// Elimina y devuelve el elemento en el índice especificado de la lista enlazada simple.
    ///
    /// # Parámetros
    /// - `index`: El índice del elemento que se desea eliminar. Debe estar en el rango `0..self.len`.
    ///
    /// # Retornos
    /// - `Ok(T)`: Si el índice es válido, devuelve el valor del elemento eliminado.
    /// - `Err(Exceptions::IndexOutOfBounds)`: Si el índice está fuera del rango válido de la lista.
    /// - `Err(Exceptions::NoSuchElement)`: Si ocurre un error inesperado al intentar encontrar o eliminar el elemento.
    ///
    /// # Comportamiento
    /// - Si `index` es `0`, se elimina el primer nodo utilizando el método `shift`.
    /// - Para cualquier otro índice, recorre la lista hasta el nodo previo al índice, ajusta los enlaces para saltarse el nodo eliminado y lo desvincula de la lista.
    /// - La longitud de la lista (`len`) se reduce en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::from(&[10, 20, 30, 40]);
    ///
    /// // Eliminar el elemento en el índice 1.
    /// let removed = list.remove(3);
    /// assert_eq!(removed, Ok(40)); // El valor eliminado es 20.
    /// assert_eq!(list.len(), 3); // La lista ahora tiene 2 elementos.
    ///
    /// // Eliminar el elemento en el índice 1.
    /// let removed = list.remove(1);
    /// assert_eq!(removed, Ok(20)); // El valor eliminado es 20.
    /// assert_eq!(list.len(), 2); // La lista ahora tiene 2 elementos.
    /// assert_eq!(list.get(1), Ok(&30)); // El elemento en índice 1 ahora es 30.
    ///
    /// // Eliminar el elemento en el índice 0.
    /// let removed = list.remove(0);
    /// assert_eq!(removed, Ok(10)); // El valor eliminado es 10.
    /// assert_eq!(list.len(), 1); // La lista ahora tiene 1 elemento.
    /// assert_eq!(list.get(0), Ok(&30)); // El elemento en índice 0 ahora es 30.
    ///
    /// // Eliminar el elemento en el índice 0.
    /// let removed = list.remove(0);
    /// assert_eq!(removed, Ok(30)); // El valor eliminado es 30.
    /// assert!(list.is_empty()); // La lista ahora está vacía.
    ///
    /// // Intentar eliminar un índice fuera de rango.
    /// assert!(list.remove(0).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::IndexOutOfBounds` si `index` es mayor o igual a la longitud de la lista (`len`).
    /// - `Exceptions::NoSuchElement` si ocurre un error inesperado al intentar acceder o eliminar el nodo.
    ///
    /// # Notas
    /// - Este método tiene un costo lineal (`O(n)`), ya que requiere recorrer la lista hasta el índice especificado.
    /// - Es útil para listas donde las operaciones de eliminación no son frecuentes.
    pub fn remove(&mut self, index: usize) -> Result<T, Exceptions> {
        match self.head.take() {
            Some(mut node) => {
                if index == 0 {
                    self.head = Some(node);
                    self.shift()
                } else {
                    let mut list: Self = Self::default();
                    list.push(node.get().to_owned());
                    let mut pred = &mut node;
                    let mut i: usize = 0;
                    let mut last_node: Option<Box<Node<T>>> = None;
                    while let Some(ref mut current) = pred.get_next_mut() {
                        if i == index - 1 {
                            last_node = Some(current.clone());
                        } else {
                            list.push(current.get().to_owned());
                        }
                        i += 1;
                        pred = current;
                    }
                    match last_node {
                        Some(last_node) => {
                            self.head = list.head;
                            self.len -= 1;
                            Ok(last_node.get().to_owned())
                        }
                        None => Err(Exceptions::NoSuchElement(String::from("Element not found"))),
                    }
                }
            }
            None => Err(Exceptions::NoSuchElement(String::from("The list is empty"))),
        }
    }

    /// Devuelve la cantidad de elementos almacenados actualmente en la lista enlazada simple.
    ///
    /// # Retornos
    /// - `usize`: El número de elementos actualmente presentes en la lista.
    ///
    /// # Comportamiento
    /// - La longitud (`len`) representa el número de elementos almacenados en la lista.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::from(&[1, 2, 3]);
    ///
    /// // La lista tiene 3 elementos.
    /// assert_eq!(list.len(), 3);
    ///
    /// // Después de agregar un elemento, la longitud aumenta.
    /// list.push(4);
    /// assert_eq!(list.len(), 4);
    ///
    /// // Eliminar un elemento reduce la longitud.
    /// list.remove(0).unwrap();
    /// assert_eq!(list.len(), 3);
    /// ```
    ///
    /// # Notas
    /// - Para verificar si la lista está vacío, utiliza el método `is_empty`.
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Verifica si la lista enlazada simple está vacía.
    ///
    /// # Retornos
    /// - `true`: Si la longitud actual de la lista (`len`) es `0`.
    /// - `false`: Si la lista contiene uno o más elementos.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let mut list = SinglyLinkedList::default();
    ///
    /// // Una lista recién creado está vacío.
    /// assert!(list.is_empty());
    ///
    /// // Después de agregar un elemento, ya no está vacía.
    /// list.push(10);
    /// assert!(!list.is_empty());
    ///
    /// // Al eliminar todos los elementos, vuelve a estar vacío.
    /// list.remove(0).unwrap();
    /// assert!(list.is_empty());
    /// ```
    ///
    /// # Notas
    /// - Este método es equivalente a comprobar si `len() == 0`.
    /// - Útil para validar condiciones antes de realizar operaciones en la lista.
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Crea un iterador para recorrer los elementos de la lista enlazada simple.
    ///
    /// # Retornos
    /// - `SinglyLinkedListIterator<T>`: Un iterador que permite recorrer los elementos de la lista en orden desde el primer nodo hasta el último.
    ///
    /// # Comportamiento
    /// - El iterador devuelve referencias inmutables a los valores almacenados en los nodos de la lista.
    /// - Los valores se recorren en el mismo orden en que están enlazados en la lista.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// let list = SinglyLinkedList::with_data(10);
    /// let mut iter = list.iter();
    ///
    /// // Recorrer los elementos de la lista.
    /// assert_eq!(iter.next(), Some(10));
    /// assert_eq!(iter.next(), None); // No hay más elementos.
    ///
    /// // Usar un bucle para iterar.
    /// let list = SinglyLinkedList::from(&[1, 2, 3]);
    /// for value in list.iter() {
    ///     println!("{}", value);
    /// }
    /// ```
    ///
    /// # Notas
    /// - El iterador es inmutable, por lo que no permite modificar los elementos de la lista.
    /// - Si necesitas iterar y modificar los valores, deberás implementar o usar un iterador mutable adicional.
    /// - La creación del iterador es una operación de tiempo constante (`O(1)`).
    /// - Este método está marcado como `#[must_use]`, lo que indica que su valor de retorno debe ser utilizado; de lo contrario, se generará una advertencia.
    #[must_use]
    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        SinglyLinkedListIterator::new(self.head.clone())
    }
}

impl<T: Copy + PartialEq> SinglyLinkedList<T> {
    /// Inserta un nuevo elemento después del primer nodo que contiene el valor especificado.
    ///
    /// # Parámetros
    /// - `pred_value`: El valor del nodo predecesor del cual se insertará el nuevo elemento.
    /// - `value`: El valor que se desea insertar en la lista.
    ///
    /// # Retornos
    /// - `Ok(())`: Si se encuentra el nodo con `pred_value` y se inserta el nuevo valor correctamente.
    /// - `Err(Exceptions::NoSuchElement)`: Si no se encuentra ningún nodo con el valor `pred_value`.
    ///
    /// # Comportamiento
    /// - Recorre la lista en busca del primer nodo que contenga el valor `pred_value`.
    /// - Crea un nuevo nodo con `value` y ajusta los enlaces para colocarlo inmediatamente después del nodo encontrado.
    /// - Incrementa la longitud de la lista (`len`) en 1.
    ///
    /// # Ejemplo
    /// ```
    /// # use linked_list::SinglyLinkedList;
    /// # use exceptions::Exceptions;
    /// let mut list = SinglyLinkedList::from(&[10, 20]);
    ///
    /// // Insertar un elemento después del nodo con valor 10.
    /// assert_eq!(list.insert_after(10, 15), Ok(()));
    /// assert_eq!(list.get(1), Ok(&15));
    /// assert_eq!(list.len(), 3);
    ///
    /// // Intentar insertar después de un valor que no existe.
    /// assert!(list.insert_after(4, 30).is_err());
    /// ```
    ///
    /// # Errors
    /// Este método retornará:
    /// - `Exceptions::NoSuchElement` si no se encuentra ningún nodo con el valor `pred_value`.
    ///
    /// # Notas
    /// - Este método tiene un costo lineal (`O(n)`), ya que requiere recorrer la lista para buscar el nodo especificado.
    /// - Es útil para listas donde es necesario insertar elementos relativos a un valor específico.
    pub fn insert_after(&mut self, pred_value: T, value: T) -> Result<(), Exceptions> {
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(ref mut node) = pred {
            if *node.get() == pred_value {
                let mut new_node = Node::new(value);
                new_node.set_next(node.get_next_mut().take());
                node.set_next(Some(Box::new(new_node)));
                self.len += 1;
                return Ok(());
            }
            pred = node.get_next_mut();
        }
        Err(Exceptions::NoSuchElement(String::from(
            "Predecessor not found",
        )))
    }
}

impl<T: Clone, const N: usize> From<&[T; N]> for SinglyLinkedList<T> {
    fn from(values: &[T; N]) -> Self {
        let mut list: Self = Self::default();
        for value in values.iter().rev() {
            list.unshift(value.to_owned());
        }
        list
    }
}

impl<T: Clone> From<&[T]> for SinglyLinkedList<T> {
    fn from(values: &[T]) -> Self {
        let mut list: Self = Self::default();
        for value in values.iter().rev() {
            list.unshift(value.to_owned());
        }
        list
    }
}

impl<T: Clone> From<Vec<T>> for SinglyLinkedList<T> {
    fn from(values: Vec<T>) -> Self {
        let mut list: Self = Self::default();
        for value in values.iter().rev() {
            list.unshift(value.to_owned());
        }
        list
    }
}

impl<T: Clone> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> IntoIterator for &SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = SinglyLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone + Debug> Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "[")?;
        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value:?}")?;
        }
        write!(f, "]")
    }
}
