use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use regex::Regex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut accumulator: u8 = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        accumulator += 1;
        handle_connection(stream, accumulator);
    }
}

fn handle_connection(mut stream: TcpStream, num: u8) {
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
            if (post_rq.len() == 31 && !re.is_match(&post_rq[6..22])) => scrape_board(&post_rq[6..22]),
        Some(_) => "Error: Incomplete Table",
        _ => "Error: Generic"
    };

    let length = contents.len();

    let response = format!("{status_line}\r\n{other_stuff}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    println!("Response:\n{}", response);

    stream.write_all(response.as_bytes()).unwrap();
}

//board is guaranteed to be 16 characters and only containing a-z or A-Z
fn scrape_board(board: &str) -> &str{
    &"it workey"
}