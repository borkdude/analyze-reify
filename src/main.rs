use tree_sitter::{Parser, Language, Node};
use std::fs::File;
use std::io::prelude::*;
use std::env;

extern "C" { fn tree_sitter_clojure() -> Language; }

fn node_text<'a>(node: Node<'a>, bytes: &'a [u8]) -> &'a str {
    node.utf8_text(bytes).unwrap()
}

fn print_node(node: Node, bytes: &[u8]) {
    println!("{}",node_text(node, bytes));
}

fn print_reify(node: Node, bytes: &[u8]) {
    let text = node_text(node, &bytes);
    if text == "reify" {
        let prev_sibling = node.prev_sibling().unwrap();
        let text = node_text(prev_sibling, &bytes);
        if text == "(" {
            print_node(node.next_sibling().unwrap(), &bytes);
        }
    }
    let kind = node.kind();
    if (kind == "list") || (kind == "source_file") {
        let child_count = node.child_count();
        //println!("{:?}", node.kind());
        for child_num in 0..child_count {
            let child = node.child(child_num).unwrap();
            print_reify(child, &bytes);
        }
    }
}

fn main() {
    let language = unsafe { tree_sitter_clojure() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    //let args: Vec<String> = vec![String::from("core.clj")];
    let args: Vec<String> = env::args().collect();
    for filename in args.iter().skip(1) {
        let mut f = File::open(filename).unwrap();
        let mut source_code = String::new();
        f.read_to_string(&mut source_code).unwrap();
        let bytes = source_code.as_bytes();
        let tree = parser.parse(&source_code, None).unwrap();
        let root_node = tree.root_node();
        print_reify(root_node, &bytes);
    }
}
