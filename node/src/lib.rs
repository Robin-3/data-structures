#[derive(Debug)]
pub struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub const fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn get(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn get_next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }

    pub fn get_next_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.next
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }

    pub fn set_next(&mut self, next: Option<Box<Node<T>>>) {
        self.next = next;
    }
}
