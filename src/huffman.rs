use bitvec::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Eq, PartialEq)]
pub enum Node {
    Nil,
    Tree(u32, Box<Node>, Box<Node>),
    Leaf(u8, u32),
}

type SerializedNodeType = u8;

const SerializedTypeNil: SerializedNodeType = 1;
const SerializedTypeLeaf: SerializedNodeType = 2;
const SerializedTypeInternal: SerializedNodeType = 3;

#[derive(Clone)]
pub struct SerializedNode {
    // indicates whether the node is a leaf node, an internal node, or Nil
    nodeType: SerializedNodeType,
    value: u8,
    frequency: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        match self {
            Node::Leaf(_, f1) => match other {
                Node::Leaf(_, f2) => f1.cmp(f2),
                _ => Ordering::Less,
            },
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn write_to_file(serialized: Vec<SerializedNode>, filepath: &str) -> io::Result<()> {
    let path = Path::new(filepath); 
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    for node in serialized {
        if let Err(e) = file.write_all(format!("{},{},{},", node.nodeType, node.value,
                                     node.frequency).as_bytes()) {
            return Err(e);
        }
    }
    Ok(())
}

// Serializes the provided tree into text
pub fn serialize_tree(root: &Node) -> Vec<SerializedNode> {
    let mut serialized = Vec::new();
    match root {
        Node::Nil => {
            serialized.push(SerializedNode{
                nodeType: SerializedTypeNil,
                frequency: 0,
                value: 0,
            });
        }
        Node::Tree(freq, left, right) => {
            let st = &mut String::from("");
            let sn = SerializedNode{
                nodeType: SerializedTypeInternal,
                frequency: *freq,
                value: 0,
            };
            serialized.push(sn);
            serialized.extend(serialize_tree(left));
            serialized.extend(serialize_tree(right));
        }
        Node::Leaf(val, freq) => {
            let sn = SerializedNode{
                nodeType: SerializedTypeLeaf,
                value: *val,
                frequency: *freq,
            };
            serialized.push(sn);
        }
    };
    serialized.to_vec()
}

pub fn deserialize_tree(repr: String) -> Node {
    // Split by comma
    let entries: Vec<&str> = repr.split(",").collect();

    if entries.len() < 3 {
        return Node::Nil
    }
    let idx = Rc::new(RefCell::new(0));
    readIntoTree(&entries, idx)
}

pub fn readIntoTree(values: &Vec<&str>, idx: Rc<RefCell<u32>>) -> Node {
    let nodeType = values[(*idx.borrow()) as usize].parse::<u8>().unwrap() as SerializedNodeType;
    match nodeType {
        SerializedTypeInternal => {
            let value = values[(*idx.borrow() + 1) as usize].parse::<u8>().unwrap();
            let freq = values[(*idx.borrow() + 2) as usize].parse::<u32>().unwrap();
            *idx.borrow_mut() += 3;
            let left = readIntoTree(values, Rc::clone(&idx));
            let right = readIntoTree(values, Rc::clone(&idx));
            Node::Tree(freq, Box::new(left), Box::new(right))
        },
        SerializedTypeNil => {
            *idx.borrow_mut() += 3;
            Node::Nil
        },
        SerializedTypeLeaf => {
            let value = values[(*idx.borrow() + 1) as usize].parse::<u8>().unwrap();
            let freq = values[(*idx.borrow() + 2) as usize].parse::<u32>().unwrap();
            *idx.borrow_mut() += 3;
            Node::Leaf(value, freq)
        },
        _ => {
            // Unrecognized node type
            Node::Nil
        }
    }
}

pub fn print_tree(root: &Node) -> String {
    let res = &mut String::from("");
    let to_add = match root {
        Node::Nil => "[ Nil ]".to_string(),
        Node::Tree(_, left, right) => {
            let st = &mut String::from("");
            st.push_str("{ ");
            st.push_str(&print_tree(left));
            st.push_str(&print_tree(right));
            st.push_str(" }");
            st.to_string()
        }
        Node::Leaf(val, freq) => {
            let st = &mut String::from("");
            st.push_str(&format!("[ val: {}, freq: {}]", val, freq));
            st.to_string()
        }
    };
    res.push_str(&to_add);
    res.to_string()
}

pub fn encode(
    root: &Node,
    encoded: &BitVec,
    hm: &mut HashMap<u8, BitVec>,
) -> HashMap<u8, BitVec> {

    if let Node::Nil = root {
        return HashMap::new();
    }

    match root {
        Node::Nil => {}
        Node::Leaf(val, _) => {
            hm.insert(*val, encoded.clone());
        }
        Node::Tree(_, left, right) => {
            let mut s1 = encoded.clone();
            let mut s2 = encoded.clone();
            s1.push(false);
            s2.push(true);
            encode(left, &s1, hm);
            encode(right, &s2, hm);
        }
    };
    hm.clone()
}

pub fn decode(root: &Node, idx: &mut i32, encoded: &BitVec) -> Vec<u8> {
    let mut result = Vec::new();
    match root {
        Node::Nil => {}
        Node::Leaf(val, _) => {
            result.push(*val);
        }
        Node::Tree(_, left, right) => {
            let (l, r) = (left, right);
            *idx += 1;
            let c = encoded[(*idx as usize)];
            if c == false {
                // false == 0 bit
                result.extend(&decode(&l, idx, encoded));
            } else {
                result.extend(&decode(&r, idx, encoded));
            }
        }
    };
    result
}

/// build_huffman_tree builds a Huffman tree for the provided text, encodes it,
/// and returns the encoded text along with the Huffman tree
/// along with the Huffman tree
pub fn build_huffman_tree(contents: &[u8], hm: &mut HashMap<u8, u32>) -> (BitVec, Node) {
    for c in contents {
        let count = hm.entry(*c).or_insert(0);
        *count += 1;
    }
    let pq = &mut BinaryHeap::new();

    for (c, freq) in hm {
        pq.push(Node::Leaf(*c, *freq));
    }

    while pq.len() > 1 {
        let fst = pq.pop().unwrap();
        let v1 = match fst {
            Node::Nil => 0,
            Node::Leaf(_, freq) => freq,
            Node::Tree(freq, _, _) => freq,
        };
        let snd = pq.pop().unwrap();
        let v2 = match snd {
            Node::Nil => 0,
            Node::Leaf(_, freq) => freq,
            Node::Tree(freq, _, _) => freq,
        };
        pq.push(Node::Tree(v1 + v2, Box::new(fst), Box::new(snd)));
    }

    let root = pq.peek().unwrap().clone();

    let em = &mut HashMap::new();
    let encode_map = encode(&root, &BitVec::new(), em);

    let mut encoded_result = BitVec::new();

    for c in contents {
        encoded_result.extend(encode_map.get(&c).unwrap().iter().copied());
    }
    (encoded_result, root)
}

pub fn encode_from_tree(contents: &[u8], root: &Node) -> BitVec {
    let em = &mut HashMap::new();
    let encode_map = encode(&root, &BitVec::new(), em);

    let mut encoded_result = BitVec::new();

    for c in contents {
        encoded_result.extend(encode_map.get(&c).unwrap().iter().copied());
    }
    encoded_result
}

pub fn decode_huffman_tree(encoded: &BitVec, root: &Node, buffer: &mut Vec<u8>) {
    let idx: &mut i32 = &mut (-1);

    while *idx < (encoded.len() - 2) as i32 {
        let res = decode(root, idx, encoded);
        for c in res {
            buffer.push(c);
        }
    }
}
