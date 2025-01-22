#[derive(Debug, PartialEq, Eq)]
pub enum Exceptions {
    IndexOutOfBounds,
    KeyNotInitialized,
    DuplicateKey,
    NoSuchElement(String),
}
