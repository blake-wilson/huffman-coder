use std::collections::HashMap;

enum Node<'a> {
    Nil,
    Tree(char, u32, Box<&'a Node<'a>>, Box<&'a Node<'a>>),
}

static NIL: &Node = &Node::Nil;

fn new_node<'a>(ch: char, freq: u32, left: &'a Node, right: &'a Node) -> Node<'a> {
    Node::Tree(ch, freq, Box::new(left), Box::new(right))
}

fn print_tree(root: &Node) -> String {
    let res = &mut String::from("");
    let to_add = match root {
        Node::Nil => "[ Nil ]".to_string(),
        Node::Tree(val, freq, left, right) => {
            let st = &mut String::from("");
            st.push_str(&format!("[ val: {}, freq: {}]", val, freq));

            st.push_str("{ ");
            st.push_str(&print_tree(left));
            st.push_str(&print_tree(right));
            st.push_str(" }");
            st.to_string()
        }
    };
    res.push_str(&to_add);
    res.to_string()
}

fn encode(root: &Node, st: &str, hm: &mut HashMap<char, String>) {
    if let data = Node::Nil {
        return;
    }

    match root {
        Nil => {}
        Node::Tree(val, freq, left, right) => {
            let (l, r) = (**left, **right);
            match (l, r) {
                (Node::Nil, Node::Nil) => {
                    hm.insert(*val, st.to_string());
                }
                (Node::Tree(ch, freq, left, right), _) => {
                    let mut s = String::from(st);
                    s.push_str("0");
                    encode(&Box::new(left), &s, hm);
                }
                (_, Node::Tree(ch, freq, left, right)) => {
                    let mut s = String::from(st);
                    s.push_str("1");
                    encode(&Box::new(right), &s, hm);
                }
            }
        }
    };
}

fn main() {
    println!("Hello, world!");

    let sub_node = &new_node('b', 2, NIL, NIL);

    let root = new_node('a', 1, sub_node, NIL);
    let hm = &mut HashMap::new();

    encode(&root, "a", hm);
    let txt = print_tree(&root);
    println!("{}", txt);
}
