#[derive(Debug, Clone)]
pub struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub const fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub const fn get(&self) -> &T {
        &self.data
    }

    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[allow(clippy::ref_option)]
    pub const fn get_next(&self) -> &Option<Box<Self>> {
        &self.next
    }

    pub fn get_next_mut(&mut self) -> &mut Option<Box<Self>> {
        &mut self.next
    }

    pub fn set(&mut self, data: T) {
        self.data = data;
    }

    pub fn set_next(&mut self, next: Option<Box<Self>>) {
        self.next = next;
    }
}
