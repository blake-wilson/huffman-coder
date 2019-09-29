mod huffman;

fn main() {
    use std::collections::HashMap;
    use std::io::{self, Read};

    // let sub_node = Node::Tree(2, Box::new(Node::Nil), Box::new(Node::Nil));
    // let root = &Node::Tree(1, Box::new(sub_node), Box::new(Node::Leaf('a', 1)));

    // let hm = &mut HashMap::new();
    // encode(root, "", hm);
    // let txt = print_tree(&root);
    // println!("{}", txt);
    // println!("{:?}", hm);
    // let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    let (encoded, root) = huffman::build_huffman_tree(&buffer);
    println!("encoded string: {}", encoded);
    let decoded = huffman::decode_huffman_tree(&encoded, &root);
    println!("decoded string: {}", decoded);
    println!(
        "compressed from {} bytes to {}",
        buffer.len(),
        encoded.len() / 8
    );
}
