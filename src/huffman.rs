use bitvec::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
pub enum Node {
    Nil,
    Tree(u32, Box<Node>, Box<Node>),
    Leaf(u8, u32),
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
        Node::Leaf(freq, val) => {
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
        encoded_result.extend(encode_map.get(&c).unwrap());
    }
    (encoded_result, root)
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
