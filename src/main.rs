use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
fn main() {
let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

for stream in listner.incoming(){
    let stream = stream.unwrap();
   
    handle_connection(stream)
}
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "http/1.1 200 ok\r\nContent length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }else{
        let status_line ="HTTP/1.1 404 not found";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!(
        "http/1.1 200 ok\r\nContent length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    }
    
}
