use std::collections::HashMap;
use num::pow;

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
        byte.push_str((val / pow(2,i)).to_string().as_str());
        val = val % pow(2,i);
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
