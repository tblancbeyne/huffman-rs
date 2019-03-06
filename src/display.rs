use std::collections::HashMap;

use crate::{Node, Symbol};

pub fn display_leafs(leafs: &Vec<Node>) {
    println!("Displaying the list of symbols and their frequency");
    for element in leafs {
        match element.symbol {
            Some(val) => println!("{:?} : {}", val_to_string(&val), element.frequency),
            None => unreachable!("Empty leaf"),
        }
    }
    println!();
}

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
    println!("Displaying the Huffman tree");
    display_tree_aux(tree, 0, &mut Vec::new());
    println!();
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

        Some(None) => println!("({}) \"EOF\"", tree.frequency),

        Some(Some(val)) => println!(
            "({}) {:?}",
            tree.frequency,
            std::str::from_utf8(&[val]).expect("Error")
        ),
    }
}

pub fn display_code(code: &HashMap<Symbol, Vec<bool>>) {
    println!("Displaying the encoding");
    for element in code.iter() {
        match element.0 {
            Some(val) => {
                let slice = &[*val];
                let string = std::str::from_utf8(slice).expect("Error");
                print!("{:?}", string);
            }
            None => print!("\"EOF\""),
        }

        print!(": ");

        for val in element.1 {
            if *val {
                print!("1");
            } else {
                print!("0");
            }
        }

        println!();
    }
    println!();
}

fn val_to_string(val: &Symbol) -> String {
    match val {
        Some(val) => std::str::from_utf8(&[*val]).expect("Error").to_owned(),
        None => "EOF".to_owned(),
    }
}

pub fn display_text(text: &Vec<u8>) {
    println!("Displaying the content of the file");
    for element in text {
        let slice = &[*element];
        let string = std::str::from_utf8(slice).expect("Error");
        print!("{}", string);
    }
    println!();
}
