use crate::Step;
use crate::{Node, Symbol};

pub fn decode_tree(encoded_tree: &[bool], table: &Vec<Symbol>) -> Node {
    decode_tree_aux(encoded_tree, table, 0).0
}

fn decode_tree_aux<'a>(
    encoded_tree: &'a [bool],
    table: &Vec<Symbol>,
    index: usize,
) -> (Node, &'a [bool], usize) {
    let mut tree = Node {
        symbol: None,
        frequency: 0,
        left: None,
        right: None,
    };

    let (encoded_tree, index) = if !encoded_tree[0] {
        let (left, encoded_tree_left, index) =
            decode_tree_aux(&encoded_tree[1..encoded_tree.len()], table, index);
        tree.left = Some(Box::new(left));
        let (right, encoded_tree_right, index) = decode_tree_aux(encoded_tree_left, table, index);
        tree.right = Some(Box::new(right));
        (encoded_tree_right, index)
    } else {
        tree.symbol = Some(table[index]);
        (&encoded_tree[1..encoded_tree.len()], index + 1)
    };

    (tree, encoded_tree, index)
}

pub fn decode_text(tree: &Node, text: &[bool]) -> Vec<u8> {
    let mut curr_tree = tree;
    let mut decoded_text = Vec::new();

    for bit in text {
        if *bit {
            curr_tree = match curr_tree.right {
                Some(ref node) => &*node,
                None => unreachable!("Tree is not perfect"),
            };
        } else {
            curr_tree = match curr_tree.left {
                Some(ref node) => &*node,
                None => unreachable!("Tree is not perfect"),
            };
        }

        match curr_tree.symbol {
            None => (),
            Some(None) => break,
            Some(Some(val)) => {
                decoded_text.push(val);
                curr_tree = tree;
            }
        }
    }

    decoded_text
}

pub fn bools_to_bytes(text: &Vec<bool>) -> Vec<u8> {
    let mut i = 0;
    let mut curr_byte = 0;
    let mut bytes = Vec::new();

    for val in text {
        curr_byte += if *val { 1 } else { 0 };

        if i == 7 {
            i = 0;
            bytes.push(curr_byte);
            curr_byte = 0;
        } else {
            i += 1;
            curr_byte *= 2;
        }
    }

    while i < 7 {
        curr_byte *= 2;
        i += 1;
    }

    bytes
}

pub fn byte_to_bools(mut byte: u8) -> Vec<bool> {
    let mut bools = Vec::new();

    for i in (0..8).rev() {
        bools.push(if byte / 2u8.pow(i) == 1 { true } else { false });
        byte = byte % 2u8.pow(i);
    }

    bools
}

pub fn update_table(table: &mut Vec<Symbol>, byte: u8) -> Step {
    let mut step = Step::Table;
    if match table.last() {
        Some(Some(val)) => *val == byte && table.len() > 1,
        Some(None) => unreachable!("No EOF symbol yet!"),
        None => false,
    } {
        step = Step::Tree;
        let pos = table[0].unwrap();
        table.remove(0);
        table.insert(pos as usize, None);
    } else {
        table.push(Some(byte));
    }

    step
}

pub fn read_tree(tree: &mut Vec<bool>, byte: u8) -> (Step, Option<usize>) {
    let byte = byte_to_bools(byte);
    let mut nb_leaf = 0;
    let mut nb_node = 0;
    let mut i = 0;

    for bit in tree.as_mut_slice() {
        if *bit {
            nb_leaf += 1;
        } else {
            nb_node += 1;
        }
    }

    for bit in byte {
        i += 1;
        tree.push(bit);
        if bit {
            nb_leaf += 1;
            if nb_leaf == nb_node + 1 {
                return (Step::Text, Some(i));
            }
        } else {
            nb_node += 1;
        }
    }

    (Step::Tree, None)
}
