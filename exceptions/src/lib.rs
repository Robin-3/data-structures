#[derive(Debug)]
pub enum Exceptions {
    IndexOutOfBounds,
    NoSuchElement(String),
}
