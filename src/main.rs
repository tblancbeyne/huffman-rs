use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use huffman_rs::decode::{
    bools_to_bytes, byte_to_bools, decode_text, decode_tree, read_tree, update_table,
};
use huffman_rs::display::{display_code, display_leafs, display_text, display_tree};
use huffman_rs::encode::{compress, create_tree, encode_bytes, encode_symbols, encode_tree};
use huffman_rs::Node;
use huffman_rs::Step;
use huffman_rs::BUFFER_SIZE;

fn main() {
    /////////////////////////////////////////////////
    ////////////////// COMPRESSION //////////////////
    /////////////////////////////////////////////////

    // Checking that something is given as first argument
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("You need to give a filename!");
            exit(1);
        }
    };

    // Checking if running in verbose mode
    let verbose = match env::args().nth(2) {
        Some(f) => {
            if f == "--v" {
                true
            } else {
                false
            }
        }
        None => false,
    };

    if verbose {
        println!("Running in verbose mode");
    }

    // Opening file whose name is given as first argument
    let mut file = File::open(&filename).expect("Error opening file");

    // Constructing a buffer to use while reading the file
    let mut buffer = [0; BUFFER_SIZE];

    // Constructing a hasmap to store the frequencies of each symbol
    let mut freqs = HashMap::new();

    // Pseudo EOF character
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
    if verbose {
        display_leafs(&leafs);
    }

    // Creating the Huffman tree
    let root = create_tree(leafs);

    // Displaying the tree
    if verbose {
        display_tree(&root);
    }

    // Encoding the bytes thanks to the tree
    let (encoding, symbols) = encode_bytes(&root);

    // Displaying the code and the symbols
    if verbose {
        display_code(&encoding);
    }

    // Encoding the tree
    let mut encoded_tree = encode_tree(&root);

    // Compressing the text
    let mut compressed_text = compress(&encoding, &filename);

    // Encoding the symbols
    let encoded_symbols = encode_symbols(&symbols);

    // Constructing the final text and content
    let mut content = encoded_symbols;
    let mut text = Vec::new();
    text.append(&mut encoded_tree);
    text.append(&mut compressed_text);
    content.extend_from_slice(&bools_to_bytes(&text));

    // Opening destination file
    let mut dest_file = File::create(format!("{}.hff", filename)).expect("Error opening file");

    // Writing in destination file
    dest_file.write(&content).expect("Error writing in file");

    /////////////////////////////////////////////////
    ///////////////// DECOMPRESSION /////////////////
    /////////////////////////////////////////////////

    // Opening encoded file
    let mut src_file = File::open(format!("{}.hff", filename)).expect("Error opening file");

    // Variables to store the text, tree and symbol table
    let mut table = Vec::new();
    let mut tree = Vec::new();
    let mut text = Vec::new();

    // Reading the file
    let mut step = Step::Table;
    loop {
        // Reading BUFFER_SIZE bytes in the file
        let read = src_file.read(&mut buffer).expect("Error reading file");

        // For each read byte, updating the correct component
        for byte in buffer.iter().take(read) {
            match step {
                Step::Table => {
                    step = update_table(&mut table, *byte);
                }

                Step::Tree => {
                    let (curr_step, i) = read_tree(&mut tree, *byte);
                    step = curr_step;

                    match i {
                        Some(val) => {
                            let byte = byte_to_bools(*byte);

                            for i in val..8 {
                                text.push(byte[i]);
                            }
                        }
                        None => (),
                    }
                }

                Step::Text => {
                    let byte = byte_to_bools(*byte);

                    for bit in byte {
                        text.push(bit);
                    }
                }
            }
        }

        // If we reached the end of the file, get out
        if read == 0 {
            break;
        }
    }

    // Decoding the tree
    let tree = decode_tree(&tree, &table);

    if verbose {
        display_tree(&tree);
    }

    // Decoding the text
    let text = decode_text(&tree, &text);

    if verbose {
        display_text(&text);
    }
}
