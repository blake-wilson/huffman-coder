mod huffman;
use std::collections::HashMap;
use std::io::{self, Read, BufReader, Write};
use std::str;
use std::fs;

const KILOBYTE: i32 = 8 * 1024;

fn main() {

    // let sub_node = Node::Tree(2, Box::new(Node::Nil), Box::new(Node::Nil));
    // let root = &Node::Tree(1, Box::new(sub_node), Box::new(Node::Leaf('a', 1)));

    // let hm = &mut HashMap::new();
    // encode(root, "", hm);
    // let txt = print_tree(&root);
    // println!("{}", txt);
    // println!("{:?}", hm);
    // let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";
    
    // io::stdin().read_to_string(&mut buffer);
    let mut reader = BufReader::new(io::stdin());
    let buffer = &mut[0; (KILOBYTE * 4) as usize];
    let hm: &mut HashMap<u8, u32> = &mut HashMap::new();
    while reader.read(buffer).unwrap() != 0 {
        let res = build_tree(buffer, hm);
        io::stdout().write_all(format!("result: {}", res).as_bytes());
        io::stdout().write_all(
            format!("compressed from {} bytes to {}\n", 
                    buffer.len(), res.len() / 8).as_bytes(),
        );
    }
    // serialize huffman tree
}

fn build_tree(input: &mut [u8], hm: &mut HashMap<u8, u32>) -> String {
        // let (encoded, root) = huffman::build_huffman_tree(input, hm);
        // println!("encoded string: {}", encoded);

        // let serialized = huffman::serialize_tree(&root);
        // huffman::write_to_file(serialized, "tree.txt");

        // let out_buffer = &mut Vec::new();
        // huffman::decode_huffman_tree(&encoded, &root, out_buffer);
        // let result_str = str::from_utf8(&out_buffer);
        // match result_str {
        //     Ok(res) => res.to_string(),
        //     Err(err) => String::from("error"),
        // }
        //

        let fileContents = fs::read_to_string("tree.txt").unwrap();
        let root = huffman::deserialize_tree(fileContents);
        let encoded = huffman::encode_from_tree(input, &root);
        let result = huffman::print_tree(&root);
        println!("result: {}", result);

        let out_buffer = &mut Vec::new();
        huffman::decode_huffman_tree(&encoded, &root, out_buffer);
        let result_str = str::from_utf8(&out_buffer);
        match result_str {
            Ok(res) => res.to_string(),
            Err(err) => String::from("error"),
        }
}
