use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, usize,
};
use regex::Regex;
use tree_builder::Node;

mod tree_builder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut tree: Vec<tree_builder::Node> = tree_builder::build_tree();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &tree);
    }
}

fn handle_connection(mut stream: TcpStream, tree: &Vec<tree_builder::Node>) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line: &str = "HTTP/1.1 200 OK";
    
    let origin: &str = match http_request.get(1) {
        Some(host) => &host[6..],
        _ => stringify!("*")
    };

    let other_stuff = format!("Access-Control-Allow-Origin: *");

    let re = Regex::new(r"[^a-z]gi").unwrap();

    let contents: &str = match http_request.get(0) {
        Some(post_rq) 
            if (post_rq.len() == 31 && !re.is_match(&post_rq[6..22])) => scrape_board(&post_rq[6..22], tree),
        Some(_) => "Error: Incomplete or Invalid Table",
        _ => "Error: Generic"
    };

    let length = contents.len();

    let response = format!("{status_line}\r\n{other_stuff}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    println!("Response:\n{}", response);

    stream.write_all(response.as_bytes()).unwrap();
}

//board is guaranteed to be 16 characters and only containing a-z or A-Z
fn scrape_board<'a>(board: &str, tree: &Vec<Node>) -> &'a str{
   "wow, response"
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position(i8, i8);

fn scraper_worm(curr_pos: Position, word_progress: String, board: &[char; 16],
    position_progress: &Vec<Position>, found_words: &mut Vec<String>, tree: &Vec<Node>, tree_node: usize) {
    
    let mut prev = position_progress.clone();
    prev.push(curr_pos);

    for relx in 0..3 {
        for rely in 0..3 {
            let new_position: Position = Position(curr_pos.0 + relx - 1, curr_pos.1 + rely - 1);
            if new_position.0 >= 0 && new_position.1 >= 0 && new_position.0 <= 3 && new_position.1 <= 3 {
                if !prev.contains(&new_position) {
                    let next_letter: &char = &board[(new_position.1 * 4 + new_position.0) as usize];
                    let current_node: &Node = tree.get(tree_node).unwrap();
                    if current_node.children.contains_key(next_letter) {
                        let next_node_index: usize = *current_node.children.get(next_letter).unwrap();
                        let next_node: &Node = tree.get(next_node_index).unwrap();

                        let mut new_word_progress: String = word_progress.clone();
                        new_word_progress.push(*next_letter);

                        if !found_words.contains(&new_word_progress) && next_node.is_word {
                            println!("Pushing: {}", &new_word_progress);
                            found_words.push(new_word_progress.clone());
                        }
                        scraper_worm(
                            new_position, new_word_progress, &board, &position_progress,
                            found_words, tree, next_node_index
                        );
                    }
                }
            }
        }
    }
}