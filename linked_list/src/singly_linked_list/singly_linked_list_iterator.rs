use super::node::Node;

pub struct SinglyLinkedListIterator<T: Clone> {
    current: Option<Box<Node<T>>>,
}

impl<T: Clone> SinglyLinkedListIterator<T> {
    pub fn new(head_node: Option<Box<Node<T>>>) -> Self {
        Self {
            current: head_node,
        }
    }
}

impl<T: Clone> Iterator for SinglyLinkedListIterator<T> {
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
