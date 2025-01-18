#[derive(Debug, PartialEq, Eq)]
pub enum Exceptions {
    IndexOutOfBounds,
    NoSuchElement(String),
}
