pub mod decode;
pub mod display;
pub mod encode;

pub struct Node {
    pub symbol: Option<Symbol>,
    pub frequency: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub type Symbol = Option<u8>;

pub enum Step {
    Table,
    Tree,
    Text,
}

// Size of the buffer when reading a file
pub const BUFFER_SIZE: usize = 1024;
