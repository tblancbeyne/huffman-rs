use std::collections::HashMap;

use crate::{Node, Symbol};

fn update_bars(rank: usize, bars: &mut Vec<bool>, val: bool) {
    if bars.len() == rank {
        bars.push(val);
    } else {
        bars[rank] = val;
    }
}

fn align(rank: usize, bars: &Vec<bool>) {
    for i in 0..rank {
        if bars[i] {
            print!(" │");
        } else {
            print!("  ");
        }
        print!("      ");
    }
}

pub fn display_tree(tree: &Node) {
    display_tree_aux(tree, 0, &mut Vec::new())
}

fn display_tree_aux(tree: &Node, rank: usize, bars: &mut Vec<bool>) {
    match tree.symbol {
        None => {
            println!("({})", tree.frequency);

            update_bars(rank, bars, true);
            align(rank, bars);

            print!(" ├─ 0 ─ ");
            display_tree_aux(
                match tree.left {
                    Some(ref node) => &*node,
                    None => panic!("Tree is not perfect!"),
                },
                rank + 1,
                bars,
            );

            update_bars(rank, bars, false);
            align(rank, bars);

            print!(" └─ 1 ─ ");
            display_tree_aux(
                match tree.right {
                    Some(ref node) => &*node,
                    None => panic!("Tree is not perfect!"),
                },
                rank + 1,
                bars,
            );
        }

        Some(None) => println!("({}) \"\\$\"", tree.frequency),

        Some(Some(val)) => println!(
            "({}) {:?}",
            tree.frequency,
            std::str::from_utf8(&[val]).expect("Error")
        ),
    }
}

pub fn display_decoded_tree(tree: &Node) {
    display_decoded_tree_aux(tree, 0, &mut Vec::new())
}

fn display_decoded_tree_aux(tree: &Node, rank: usize, bars: &mut Vec<bool>) {
    match tree.symbol {
        None => {
            match tree.left {
                Some(ref node) => {
                    update_bars(rank, bars, true);
                    align(rank, bars);
                    println!(" ├─ 0 ─ ");
                    display_decoded_tree_aux(&*node, rank + 1, bars);
                },
                None => (),
            };

            match tree.right {
                Some(ref node) => {
                    update_bars(rank, bars, false);
                    align(rank, bars);
                    println!(" └─ 1 ─ ");
                    display_decoded_tree_aux(&*node, rank + 1, bars);
                },
                None => (),
            };
        }

        Some(None) => println!("({}) \"\\$\"", tree.frequency),

        Some(Some(val)) => println!(
            "({}) {:?}",
            tree.frequency,
            std::str::from_utf8(&[val]).expect("Error")
        ),
    }
}

pub fn display_code(code: &HashMap<Symbol, Vec<bool>>) {
    for element in code.iter() {
        match element {
            (Some(val), other) => {
                let slice = &[*val];
                let string = std::str::from_utf8(slice).expect("Error");
                println!("{:?} : {:?}", string, other);
            }
            (None, other) => {
                println!("\"\\$\" : {:?}", other);
            }
        }
    }
}
