use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
enum Node {
    Nil,
    Tree(u32, Box<Node>, Box<Node>),
    Leaf(char, u32),
}

impl<'a> Ord for Node {
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

static NIL: Node = Node::Nil;

fn print_tree(root: &Node) -> String {
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

fn encode(root: &Node, st: &str, hm: &mut HashMap<char, String>) -> HashMap<char, String> {
    if let Node::Nil = root {
        return HashMap::new();
    }

    match root {
        Node::Nil => {}
        Node::Leaf(val, _) => {
            hm.insert(*val, st.to_string());
        }
        Node::Tree(_, left, right) => {
            let mut s1 = String::from(st);
            let mut s2 = String::from(st);
            s1.push_str("0");
            s2.push_str("1");
            encode(left, &s1.to_string(), hm);
            encode(right, &s2.to_string(), hm);
        }
    };
    hm.clone()
}

fn decode(root: &Node, idx: &mut i32, st: &str) -> String {
    let result = &mut String::from("");
    match root {
        Node::Nil => {}
        Node::Leaf(val, _) => {
            result.push(*val);
        }
        Node::Tree(_, left, right) => {
            let (l, r) = (left, right);
            *idx += 1;
            let c = st.chars().nth(*idx as usize).unwrap();
            if c == '0' {
                result.push_str(&decode(&l, idx, st));
            } else {
                result.push_str(&decode(&r, idx, st));
            }
        }
    };
    result.to_string()
}

fn print_encoding_map(map: HashMap<char, String>) {
    for (c, code) in map {
        println!("Huffman code is {}: {}", c, code);
    }
}

fn build_huffman_tree(text: &str) {
    let hm = &mut HashMap::new();
    for c in text.chars() {
        let count = hm.entry(c).or_insert(0);
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

    let root = pq.peek().unwrap();

    let encoded_string = &mut String::from("");

    let em = &mut HashMap::new();
    let encode_map = encode(root, "", em);
    print_encoding_map(encode_map.clone());

    for c in text.chars() {
        encoded_string.push_str(encode_map.get(&c).unwrap());
    }

    let idx: &mut i32 = &mut (-1);

    let decoded = &mut String::from("");
    while *idx < (encoded_string.len() - 2) as i32 {
        let res = &decode(root, idx, encoded_string);
        decoded.push_str(res);
    }
    println!("Decoded string: {}", decoded);
}

fn main() {
    let sub_node = Node::Tree(2, Box::new(Node::Nil), Box::new(Node::Nil));
    let root = &Node::Tree(1, Box::new(sub_node), Box::new(Node::Leaf('a', 1)));
    let hm = &mut HashMap::new();

    encode(root, "", hm);
    let txt = print_tree(&root);
    println!("{}", txt);
    println!("{:?}", hm);
    // build_huffman_tree("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum");
    build_huffman_tree("Huffman coding is a data compression algorithm.");
}
