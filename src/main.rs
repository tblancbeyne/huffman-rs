use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::collections::HashMap;

use huffman_rs::{Node, Symbol};
use huffman_rs::encode::{create_tree, encode_bytes, encode_tree, encode_symbols};
use huffman_rs::decode::{decode_tree};
use huffman_rs::display::{display_tree, display_code, display_decoded_tree};

// Size of the buffer when reading a file
const BUFFER_SIZE: usize = 1024;

fn compress<P: AsRef<Path>>(code: &HashMap<Symbol, Vec<bool>>, filename: P) -> Vec<bool> {
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

//fn decode_symbols(encoded_symbols: String) -> Vec<Symbol> {
//    let chars: Vec<Symbol>;
//    let curr_char = 0;
//
//    for i in 0..7 {
//    }
//
//
//    return chars;
//
//
//}

fn val_to_string(val: &Symbol) -> String {
    match val {
        Some(val) => std::str::from_utf8(&[*val])
                .expect("Error").to_owned(),
        None =>  "\\$".to_owned(),
    }
}

fn display_symbols(symbols: &Vec<Symbol>) {
    for symbol in symbols {
        println!("{:?}", val_to_string(&symbol));
    }
}

fn bools_to_bytes(text: &Vec<bool>) -> Vec<u8> {
    let mut i = 0;
    let mut curr_byte = 0;
    let mut bytes = Vec::new();

    for val in text {
        curr_byte += if *val {
            1
        } else {
            0
        };

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
    let mut file = File::open(&filename).expect("Error opening file");

    // Constructing a buffer to use while reading the file
    let mut buffer = [0; BUFFER_SIZE];

    // Constructing a hasmap to store the frequencies of each symbol
    let mut freqs = HashMap::new();

    freqs.insert(None, 0);

    // Reading the file
    loop {
        // Reading BUFFER_SIZE bytes in the file
        let read = file.read(&mut buffer).expect("Error reading file");

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
        leafs.push(Node {
            symbol: Some(*element.0),
            frequency: *element.1,
            left: None,
            right: None,
        });
    }

    // Displaying the leafs
    for element in leafs.iter() {
        match element.symbol {
            Some(Some(val)) => println!("{:?} : {}", val, element.frequency),
            Some(None) => println!("-1 : {}", element.frequency),
            None => panic!("Empty leaf"),
        }
    }

    // Creating the Huffman tree
    let root = create_tree(leafs);

    // Displaying the tree
    display_tree(&root);

    // Encoding the bytes thanks to the tree
    let (encoding, symbols) = encode_bytes(&root);

    // Displaying the code
    display_code(&encoding);

    display_symbols(&symbols);

    let encoded_symbols = encode_symbols(&symbols);

    // Encoding the tree
    let mut encoded_tree = encode_tree(&root);
    println!("{:?}", encoded_tree);

    // Compressing the text
    let mut compressed_text = compress(&encoding, &filename);
    println!("{:?}", compressed_text);


    println!("{:?}", encoded_symbols);

    let mut text = Vec::new();
    text.append(&mut encoded_tree);
    text.append(&mut compressed_text);

    let mut content = encoded_symbols;
    content.extend_from_slice(&bools_to_bytes(&text));
    println!("{:?}", content);

    let mut dest_file = File::create(format!("{}.hff", filename))
        .expect("Error opening file");

    dest_file.write(&content)
        .expect("Error writing in file");


    // Opening file whose name is given as first argument
    //let mut dest_filename = filename;
    //dest_filename.push_str(".hff");
    // TODO : write in a file
    // TODO : read a file

    //let decoded_symbols = decode_symbols(encoded_symbols);

    //let decoded_tree = decode_tree(&encoded_tree);//, decoded_symbols);

    //display_decoded_tree(&decoded_tree);
}
