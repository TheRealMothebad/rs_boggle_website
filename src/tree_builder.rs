use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

//for a given word, divide it into sub words: h, he, hel, hell, hello

//for each subword, if it doesnt exist put it in a hashmap

//last letter points to one of the centralized 26 ending letters

//a node points to all possible endings for a given subword

#[derive(Debug)]
pub struct Node {
    pub letter: char,
    pub children: HashMap<char, usize>,
    pub is_word: bool
}

impl Node {
    fn new(letter: char) -> Self {
        Node{letter: letter, children: HashMap::new(), is_word: false}
    }
}

pub fn build_tree() -> Vec<Node> {
    std::thread::Builder::new().stack_size(32 * 1024).spawn(||{

        let path: &Path = Path::new("src/word-list.txt");

        let tree: Vec<Node> = gen_tree(path);

        check_tree(&tree, path);

        //println!("{:?}", tree);
        //pretty_print_tree(&tree);

        tree
    }).unwrap().join().unwrap()

}

fn get_node_letter(node_location: usize, next_char: &char, tree: &Vec<Node>) -> usize {
    *tree.get(node_location).expect("get node from vec failed").children.get(next_char).expect("failed to get child from node")
}

fn gen_tree(file_path: &Path) -> Vec<Node> {
    let file: File = match File::open(file_path) {
        Err(e) => panic!("wow bad: {}", e),
        Ok(file) => file
    };

    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut tree: Vec<Node> = init_map();

    for line in buf_reader.lines() {
        match line {
            Err(e) => panic!("Error in line buffer: {}", e),
            Ok(line) => add_word_to_tree(line.as_str(), &mut tree)
        }
    }

    for i in 0..26 {
        recursively_prune_ends(i, &mut tree);
    }

    tree
}

fn add_word_to_tree(line: &str, tree: &mut Vec<Node>) {
    let line_chars: Vec<char> = line.chars().collect();
    let mut curr_node: usize = char_to_usize(line_chars[0]);

    //for each letter in the line
    for i in 1..line.len() {
        if tree[curr_node].children.contains_key(&line_chars[i]) {
            curr_node = tree[curr_node].children[&line_chars[i]];
        }
        else {
            tree.push(Node::new(line_chars[i]));
            let curr_tree_len = tree.len();
            tree[curr_node].children.insert(line_chars[i], curr_tree_len - 1);
            curr_node = curr_tree_len - 1;
        }
    }

    tree[curr_node].is_word = true;
    //println!("added {line} to tree")
}

fn recursively_prune_ends(node_location: usize, tree: &mut Vec<Node>) {

    for iterator in 0..26 {
        let key: char = usize_to_char(iterator);
        if tree.get(node_location).unwrap().children.contains_key(&key) {
            let val = *tree.get(node_location).unwrap().children.get(&key).unwrap();
            if tree.get(val).unwrap().children.is_empty() {
                tree.get_mut(node_location).expect("bad").children.insert(key, char_to_usize(key) + 26);
            }
            else {
                recursively_prune_ends(val, tree)
            }
        }
    }
}

fn check_tree(tree: &Vec<Node>, file_path: &Path) {
    let file: File = match File::open(file_path) {
        Err(e) => panic!("bad file: {}", e),
        Ok(file) => file
    };

    let mut line_sizes: Vec<u8> = Vec::new();

    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut curr_node: usize;

    for line in buf_reader.lines() {
        match line {
            Err(e) => panic!("Error in line buffer: {}", e),
            Ok(line) => {
                let line_chars: Vec<char> = line.chars().collect();
                line_sizes.push(line.len() as u8);

                curr_node = char_to_usize(line_chars[0]);

                for i in 1..line.len() {
                    curr_node = get_node_letter(curr_node, &line_chars[i], &tree);
                }
            }
        }
    }
    for i in 0..26 {
        recursively_check_ends(tree, i);
    }
    println!("tree checked")
}

fn recursively_check_ends(tree: &Vec<Node>, curr_node: usize) {
    if tree.get(curr_node).unwrap().children.is_empty() && (curr_node < 26 || curr_node > 51) {
        panic!("node {} was at pos {}", tree.get(curr_node).unwrap().letter, curr_node);
    }
    for value in tree.get(curr_node).unwrap().children.values() {
        recursively_check_ends(tree, *value);
    }
}

fn init_map() -> Vec<Node> {
    let mut res: Vec<Node> = Vec::new();

    for i in 0..26 {
        res.push(Node::new(usize_to_char(i)));
    }

    for i in 0..26 {
        res.push(Node::new(usize_to_char(i)))
    }

    res
}

pub fn pretty_print_tree(tree: &Vec<Node>) {
    let mut i: u32 = 0;
    for node in tree {
        print!("{} {} | ", i, node.letter.to_ascii_uppercase());
        i = i + 1;
        for key in node.children.keys() {
            print!("\"{}\" at {}, ", key, node.children.get(key).expect("Cannot unwrap usize for pretty printing"));
        }
        println!();
    }
}

pub fn char_to_usize(c: char) -> usize {
    c as usize - 97
}

pub fn usize_to_char(u: usize) -> char {
    (u + 97) as u8 as char
}


