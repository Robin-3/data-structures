#[derive(Debug, PartialEq)]
pub enum Exceptions {
    IndexOutOfBounds,
    NoSuchElement(String),
}
