use exceptions::Exceptions;
use node::Node;
use std::fmt::{Debug, Formatter, Result as fmtResult};

pub struct LinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

pub struct LinkedListIterator<T: Clone> {
    current: Option<Box<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn with_data(data: T) -> Self {
        let node = Node::new(data);

        Self {
            head: Some(Box::new(node)),
            len: 1,
        }
    }

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

    pub fn unshift(&mut self, value: T) {
        let mut node: Node<T> = Node::new(value);
        node.set_next(self.head.take());
        self.head = Some(Box::new(node));
        self.len += 1;
    }

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

    pub fn remove(&mut self, index: usize) -> Result<T, Exceptions> {
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

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

impl<T: Copy + PartialEq> LinkedList<T> {
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

impl<T: Clone, const N: usize> From<&[T; N]> for LinkedList<T> {
    fn from(values: &[T; N]) -> Self {
        let mut list: Self = Self::default();
        for value in values {
            list.push(value.to_owned());
        }
        list
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(values: &[T]) -> Self {
        let mut list: Self = Self::default();
        for value in values {
            list.push(value.to_owned());
        }
        list
    }
}

impl<T: Clone> From<Vec<T>> for LinkedList<T> {
    fn from(values: Vec<T>) -> Self {
        let mut list: Self = Self::default();
        for value in values {
            list.push(value.to_owned());
        }
        list
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
        let node: Option<Box<Node<T>>> = self.current.clone();
        match node {
            Some(value) => {
                self.current.clone_from(value.get_next());
                Some(value.get().to_owned())
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
