use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &'static str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    if http_request[0] == "GET / HTTP/1.1" {
        let content = fs::read_to_string("hello.html").unwrap();
        let length = content.len();
        stream
            .write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{content}").as_bytes(),
            )
            .unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response =
            format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
