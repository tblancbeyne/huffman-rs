pub mod display;
pub mod encode;
pub mod decode;

pub struct Node {
    pub symbol: Option<Symbol>,
    pub frequency: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub type Symbol = Option<u8>;
