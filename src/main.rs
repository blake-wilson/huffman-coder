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

fn new_node(ch: char, freq: u32, left: Node, right: Node) -> Node {
    Node::Tree(freq, Box::new(left), Box::new(right))
}

fn print_tree(root: &Node) -> String {
    let res = &mut String::from("");
    let to_add = match root {
        Node::Nil => "[ Nil ]".to_string(),
        Node::Tree(freq, left, right) => {
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

fn encode(root: &Node, st: &str, hm: &mut HashMap<char, String>) {
    if let Node::Nil = root {
        return;
    }

    match root {
        Node::Nil => {}
        Node::Leaf(val, freq) => {
            hm.insert(*val, st.to_string());
        }
        Node::Tree(freq, left, right) => {
            let mut s1 = String::from(st);
            let mut s2 = String::from(st);
            s1.push_str("0");
            s2.push_str("1");
            encode(&Box::new(left), &s1.to_string(), hm);
            encode(&Box::new(right), &s2.to_string(), hm);
        }
    };
}

fn decode(root: Node, idx: &mut u32, st: &str) {
    match root {
        Node::Nil => {}
        Node::Leaf(val, _) => {
            println!("{}", val);
        }
        Node::Tree(freq, left, right) => {
            let (l, r) = (*left, *right);
            *idx += 1;
            let c = st.chars().nth(*idx as usize).unwrap();
            if c == '0' {
                decode(l, idx, st);
            } else {
                decode(r, idx, st);
            }
        }
    };
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
}

fn main() {
    println!("Hello, world!");

    let sub_node = Node::Tree(2, Box::new(Node::Nil), Box::new(Node::Nil));
    let root = Node::Tree(1, Box::new(sub_node), Box::new(Node::Leaf('a', 1)));
    let hm = &mut HashMap::new();

    encode(&root, "", hm);
    let txt = print_tree(&root);
    println!("{}", txt);
    println!("{:?}", hm);
}
