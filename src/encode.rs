use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::BUFFER_SIZE;
use crate::{Node, Symbol};

pub fn create_tree(mut tree: Vec<Node>) -> Node {
    while tree.len() > 1 {
        tree.sort_by(|a, b| (&(b.frequency)).cmp(&(a.frequency)));

        let leaf0 = tree.pop().unwrap();
        let leaf1 = tree.pop().unwrap();

        let freq = leaf0.frequency + leaf1.frequency;

        let new_node = Node {
            symbol: None,
            frequency: freq,
            left: Some(Box::new(leaf0)),
            right: Some(Box::new(leaf1)),
        };

        tree.push(new_node);
    }

    tree.pop().unwrap()
}

pub fn encode_byte(mut val: u8) -> String {
    let mut byte: String = "".to_owned();

    for i in (0..7).rev() {
        byte.push_str((val / 2u8.pow(i)).to_string().as_str());
        val = val % 2u8.pow(i);
    }

    return byte;
}

pub fn encode_bytes(tree: &Node) -> (HashMap<Symbol, Vec<bool>>, Vec<Symbol>) {
    let mut code = HashMap::new();
    let mut symbols = Vec::new();
    encode_bytes_aux(tree, &mut Vec::new(), &mut code, &mut symbols);

    return (code, symbols);
}

fn encode_bytes_aux(
    tree: &Node,
    curr_code: &mut Vec<bool>,
    code: &mut HashMap<Symbol, Vec<bool>>,
    symbols: &mut Vec<Symbol>,
) {
    match tree.symbol {
        None => {
            curr_code.push(false);
            encode_bytes_aux(
                match tree.left {
                    Some(ref node) => &*node,
                    None => panic!("Tree is not perfect!"),
                },
                curr_code,
                code,
                symbols,
            );
            curr_code.pop().expect("Empty curr_code !");

            curr_code.push(true);
            encode_bytes_aux(
                match tree.right {
                    Some(ref node) => &*node,
                    None => panic!("Tree is not perfect!"),
                },
                curr_code,
                code,
                symbols,
            );
            curr_code.pop().expect("Empty curr_code !");
        }

        Some(val) => {
            code.insert(val, curr_code.clone());
            symbols.push(val);
        }
    }
}

pub fn encode_tree(tree: &Node) -> Vec<bool> {
    let mut encoded_tree = Vec::new();
    match tree.symbol {
        None => {
            encoded_tree.push(false);

            match tree.left {
                Some(ref left) => {
                    encoded_tree.append(&mut encode_tree(&*left));
                }
                None => unreachable!("Tree is not perfect"),
            }

            match tree.right {
                Some(ref right) => {
                    encoded_tree.append(&mut encode_tree(&*right));
                }
                None => unreachable!("Tree is not perfect"),
            }
        }
        Some(_) => {
            encoded_tree.push(true);
        }
    }

    encoded_tree
}

pub fn encode_symbols(symbols: &Vec<Symbol>) -> Vec<u8> {
    let mut table = Vec::new();

    for (index, symbol) in symbols.iter().enumerate() {
        if symbol.is_none() {
            debug_assert!(index < 256);
            table.push(index as u8);
            break;
        }
    }

    for symbol in symbols {
        if let Some(symbol) = symbol {
            table.push(*symbol);
        }
    }

    table.push(*table.last().unwrap());

    table
}

pub fn compress<P: AsRef<Path>>(code: &HashMap<Symbol, Vec<bool>>, filename: P) -> Vec<bool> {
    let f = filename.as_ref();

    let mut file = File::open(&f).expect("Error opening file");

    // Constructing a buffer to use while reading the file
    let mut buffer = [0; BUFFER_SIZE];

    let mut text = Vec::new();

    // Reading the file
    loop {
        // Reading BUFFER_SIZE bytes in the file
        let read = file.read(&mut buffer).expect("Error reading file");

        // Updating the hashmap of frequencies
        for byte in buffer.iter().take(read) {
            text.push(*byte);
        }

        // If we reached the end of the file, get out
        if read == 0 {
            break;
        }
    }

    let mut compressed = Vec::new();

    for element in text {
        compressed.extend_from_slice(match code.get(&Some(element)) {
            Some(val) => &val,
            None => unreachable!("Empty code for symbol!"),
        });
    }

    compressed.extend_from_slice(match code.get(&None) {
        Some(val) => &val,
        None => unreachable!("Empty code for symbol!"),
    });

    compressed
}
