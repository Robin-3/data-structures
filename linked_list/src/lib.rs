use exceptions::Exceptions;
use linked_list_implementation::LinkedList;

pub fn ll_implementation() -> Result<(), Exceptions> {
    println!("Lista enlazada");
    let list: LinkedList<&str> = LinkedList::with_data("Venus");
    println!(
        "  1.1 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    let mut list: LinkedList<&str> = LinkedList::new();
    println!(
        "  1.2 Inicialización (está vacío: {0}):\n    {list:?}",
        list.is_empty()
    );
    list.insert_first("Saturno");
    println!("  2.1 Insertar al inicio (en blanco):\n    {list:?}");
    list.insert_first("Plutón");
    println!("  2.2 Insertar al inicio:\n    {list:?}");
    let pred_value = "Plutón";
    list.insert_after(pred_value, "Marte")?;
    println!("  3.1 Después de un valor (predecesor: {pred_value}):\n    {list:?}");
    let position = 2;
    list.insert_index(position, "Jupiter")?;
    println!("  3.2 Después en una posición (index: {position}):\n    {list:?}");
    list.insert_last("Urano");
    println!("  4. Insertar al final:\n    {list:?}");
    let position = 0;
    list.set(position, "Tierra")?;
    let planet = list.get(position)?;
    println!("  5. Obtener y establecer data en una posición (index: {position}, valor: {planet:?}):\n    {list:?}");
    Ok(())
}

mod linked_list_implementation {
    use exceptions::Exceptions;

    // Node in a linked list, holding a data and a reference to the next node
    #[derive(Debug)]
    pub struct Node<T: Copy> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T: Copy> Node<T> {
        const fn new(data: T) -> Self {
            Self { data, next: None }
        }
    }

    #[derive(Debug)]
    pub struct LinkedList<T: Copy> {
        head: Option<Box<Node<T>>>,
        size: usize,
    }

    impl<T: Copy> LinkedList<T> {
        pub const fn new() -> Self {
            Self {
                head: None,
                size: 0,
            }
        }

        pub fn with_data(data: T) -> Self {
            let node = Node::new(data);

            Self {
                head: Some(Box::new(node)),
                size: 1,
            }
        }

        // Return true if there are no node in the list
        pub const fn is_empty(&self) -> bool {
            self.size == 0
        }

        // Returns the value at the given index in the dynamic array
        pub fn get(&self, index: usize) -> Result<Option<T>, Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            let mut pred: &Option<Box<Node<T>>> = &self.head;
            let mut i: usize = 0;
            while let Some(ref node) = pred {
                if i == index {
                    return Ok(Some(node.data));
                }
                i += 1;
                pred = &node.next;
            }
            Ok(None)
        }

        // Sets the value at the given index in the dynamic array to the given value
        pub fn set(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
            let mut i: usize = 0;
            while let Some(ref mut node) = pred {
                if i == index {
                    node.data = value;
                    return Ok(());
                }
                i += 1;
                pred = &mut node.next;
            }
            Ok(())
        }

        // Inserts the node with specified value at the beginning of this list
        pub fn insert_first(&mut self, value: T) {
            let mut node: Node<T> = Node::new(value);
            node.next = self.head.take();
            self.head = Some(Box::new(node));
            self.size += 1;
        }

        // Inserts a new node after the node with the specified index
        pub fn insert_index(&mut self, index: usize, value: T) -> Result<(), Exceptions> {
            if index >= self.size {
                return Err(Exceptions::IndexOutOfBounds);
            }
            let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
            if index > 0 {
                let mut i: usize = 0;
                while let Some(ref mut node) = pred {
                    if i == index - 1 {
                        let mut new_node = Node::new(value);
                        new_node.next = node.next.take();
                        node.next = Some(Box::new(new_node));
                        self.size += 1;
                        return Ok(());
                    }
                    i += 1;
                    pred = &mut node.next;
                }
            } else {
                self.insert_first(value);
            }
            Ok(())
        }

        // Inserts a new node with the specified value at the end of this list
        pub fn insert_last(&mut self, value: T) {
            if self.is_empty() {
                self.head = Some(Box::new(Node::new(value)));
            }
            let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
            while let Some(ref mut node) = pred {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node::new(value)));
                    self.size += 1;
                    return;
                }
                pred = &mut node.next;
            }
        }

        // Deletes the first node from this list
        // pub fn delete_first(&mut self) -> T {}

        // Delete the first found node with the specified value
        // pub fn delete_value(&mut self, value: T) -> T {}

        // Delete the first found node with the specified index
        // pub fn delete_index(&mut self, index: usize) -> T {}

        // Deletes the last node from this list
        // pub fn delete_last(&mut self) -> T {}
    }

    impl<T: Copy + PartialEq> LinkedList<T> {
        // Inserts a new node after the node with the specified predecessor value
        pub fn insert_after(&mut self, pred_value: T, value: T) -> Result<(), Exceptions> {
            let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
            while let Some(ref mut node) = pred {
                if node.data == pred_value {
                    let mut new_node = Node::new(value);
                    new_node.next = node.next.take();
                    node.next = Some(Box::new(new_node));
                    self.size += 1;
                    return Ok(());
                }
                pred = &mut node.next;
            }
            Err(Exceptions::NoSuchElement(String::from(
                "Predecessor not found",
            )))
        }
    }
}
