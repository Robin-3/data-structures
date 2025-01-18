use exceptions::Exceptions;
use node::Node;
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

pub struct LinkedListIterator<T: Clone> {
    current: Option<Box<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    #[must_use]
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
    #[must_use]
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
                return Ok(Some(node.get().to_owned()));
            }
            i += 1;
            pred = node.get_next();
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
                node.set(value);
                return Ok(());
            }
            i += 1;
            pred = node.get_next_mut();
        }
        Ok(())
    }

    // Inserts the node with specified value at the beginning of this list
    pub fn insert_first(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);
        node.set_next(self.head.take());
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
                    new_node.set_next(node.get_next_mut().take());
                    node.set_next(Some(Box::new(new_node)));
                    self.size += 1;
                    return Ok(());
                }
                i += 1;
                pred = node.get_next_mut();
            }
        } else {
            self.insert_first(value);
        }
        Ok(())
    }

    // Inserts a new node with the specified value at the end of this list
    pub fn insert_last(&mut self, value: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node::new(value.clone())));
        }
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(ref mut node) = pred {
            if node.get_next().is_none() {
                node.set_next(Some(Box::new(Node::new(value))));
                self.size += 1;
                return;
            }
            pred = node.get_next_mut();
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

    #[must_use]
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

impl<T: Copy + PartialEq> LinkedList<T> {
    // Inserts a new node after the node with the specified predecessor value
    pub fn insert_after(&mut self, pred_value: T, value: T) -> Result<(), Exceptions> {
        let mut pred: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(ref mut node) = pred {
            if *node.get() == pred_value {
                let mut new_node = Node::new(value);
                new_node.set_next(node.get_next_mut().take());
                node.set_next(Some(Box::new(new_node)));
                self.size += 1;
                return Ok(());
            }
            pred = node.get_next_mut();
        }
        Err(Exceptions::NoSuchElement(String::from(
            "Predecessor not found",
        )))
    }
}

impl<T: Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.clone();
        match node {
            Some(value) => {
                self.current.clone_from(value.get_next());
                Some(value.get().clone())
            }
            None => None,
        }
    }
}

impl<T: Clone> IntoIterator for &LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Clone + Debug> Debug for LinkedList<T> {
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
