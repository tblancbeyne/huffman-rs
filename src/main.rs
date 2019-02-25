use colored::*;
use std::env;
use std::process::exit;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// Size of the buffer when reading a file
const BUFFER_SIZE: usize = 1024;

struct Node {
    symbol: Option<Option<u8>>,
    frequency: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn create_tree(mut tree : Vec<Node>) -> Node
{
    while tree.len() > 1 {
        tree.sort_by(|a,b| (&(b.frequency)).cmp(&(a.frequency)));

        let leaf0 = tree.pop().unwrap();
        let leaf1 = tree.pop().unwrap();

        let freq = leaf0.frequency + leaf1.frequency;

        let new_node = Node{
            symbol : None,
            frequency : freq,
            left : Some(Box::new(leaf0)),
            right : Some(Box::new(leaf1))};

        tree.push(new_node);

    }

    tree.pop().unwrap()
}

fn update_bars(rank : usize, bars : &mut Vec<bool>, val : bool) {
    if bars.len() == rank {
        bars.push(val);
    } else {
        bars[rank] = val;
    }
}

fn align(rank : usize, bars : & Vec<bool>) {
    for i in 0..rank {
        if bars[i] {
            print!(" │");
        } else {
            print!("  ");
        }
        print!("      ");
    }
}

fn display_tree(tree: &Node) {
	display_tree_aux(tree, 0, &mut Vec::new())
}

fn display_tree_aux(tree : &Node, rank : usize, bars : &mut Vec<bool>) {
    match tree.symbol {
        None => {
            println!("({})", tree.frequency);

            update_bars(rank, bars, true);
            align(rank, bars);


            print!(" ├─ 0 ─ ");
            display_tree_aux(match tree.left {
                Some(ref node) => &*node,
                None => panic!("Tree is not perfect!"),
            }, rank + 1, bars);

            update_bars(rank, bars, false);
            align(rank, bars);

            print!(" └─ 1 ─ ");
            display_tree_aux(match tree.right {
                Some(ref node) => &*node,
                None => panic!("Tree is not perfect!"),
            }, rank + 1,  bars);
        },

        Some(None) => println!("({}) \\$", tree.frequency),

        Some(Some(val)) => println!("({}) {:?}", tree.frequency, std::str::from_utf8(&[val])
			.expect("Error")),
    }
}

fn encode_bytes(tree : &Node) -> HashMap<Option<u8>, String> {
    let mut code = HashMap::new();
	encode_bytes_aux(tree, &mut String::from(""), &mut code);

    return code;
}

fn encode_bytes_aux(tree : &Node, curr_code : &mut std::string::String, code : &mut HashMap<Option<u8>, String>) {
    match tree.symbol {
        None => {
            curr_code.push_str("0");
            encode_bytes_aux(match tree.left {
                Some(ref node) => &*node,
                None => panic!("Tree is not perfect!"),
            }, curr_code, code);
            curr_code.pop()
                .expect("Empty curr_code !");

            curr_code.push_str("1");
            encode_bytes_aux(match tree.right {
                Some(ref node) => &*node,
                None => panic!("Tree is not perfect!"),
            }, curr_code, code);
            curr_code.pop()
                .expect("Empty curr_code !");
        },

        Some(val) => {
            code.insert(val, curr_code.to_string());
        },
    }
}

fn display_code(code : &HashMap<Option<u8>, String>) {
    for element in code.iter() {
        match element {
            (Some(val), other) => {
                let slice = &[*val];
                let string = std::str::from_utf8(slice).expect("Error");
                println!("{:?} : {}", string, other);
            },
            (None, other) => {
                println!("\\$ : {}", other);
            },
        }
    }
}

fn compress(code : &HashMap<Option<u8>, String>, filename : &String) {
    let mut file = File::open(filename)
        .expect("Error opening file");

    // Constructing a buffer to use while reading the file
    let mut buffer = [0; BUFFER_SIZE];

    let mut text = Vec::new();

    // Reading the file
    loop {
        // Reading BUFFER_SIZE bytes in the file
        let read = file.read(&mut buffer)
            .expect("Error reading file");

        // Updating the hashmap of frequencies
        for byte in buffer.iter().take(read) {
            text.push(*byte);
        }

        // If we reached the end of the file, get out
        if read == 0 {
            break;
        }
    }

    let mut compressed : String = "".to_owned();

    for element in text {
        compressed.push_str(match code.get(&Some(element)) {
            Some(val) => val,
            None => unreachable!("Empty code for symbol!"),
        });
    }

    compressed.push_str(match code.get(&None) {
        Some(val) => val,
        None => unreachable!("Empty code for symbol!"),
    });

    println!("{}", compressed.as_str());
}

fn main() {

    // Checking that something is given as first argument
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("You need to give a filename!");
            exit(1);
        }
    };

    // Opening file whose name is given as first argument
    let mut file = File::open(&filename)
        .expect("Error opening file");

    // Constructing a buffer to use while reading the file
    let mut buffer = [0; BUFFER_SIZE];

    // Constructing a hasmap to store the frequencies of each symbol
    let mut freqs = HashMap::new();

    freqs.insert(None, 0);

    // Reading the file
    loop {
        // Reading BUFFER_SIZE bytes in the file
        let read = file.read(&mut buffer)
            .expect("Error reading file");

        // Updating the hashmap of frequencies
        for byte in buffer.iter().take(read) {
            let entry = freqs.entry(Some(*byte)).or_insert(0);
            *entry += 1;
        }

        // If we reached the end of the file, get out
        if read == 0 {
            break;
        }
    }

    // Creating a vector of leafs
    let mut leafs = Vec::new();
    for element in freqs.iter() {
        leafs.push(Node {symbol : Some(*element.0),  frequency : *element.1, left : None, right : None});
    }

    for element in leafs.iter() {
        println!("{:?} : {}", match element.symbol {
            Some(val) => val,
            None => panic!("Empty leaf"),
        }, element.frequency);
    }

    let root = create_tree(leafs);

    display_tree(&root);
    let encoding = encode_bytes(&root);

    display_code(&encoding);

    compress(&encoding, &filename);
}

