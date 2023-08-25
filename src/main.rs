use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let stream = _stream.unwrap();
        println!("Connection established!");

        thread::spawn(||{handleStream(stream)});

    }
}

fn handleStream(mut stream: TcpStream){

    let mut buf  = [0;512];

    stream.read(&mut buf);

    let get = b"GET/HTTP/1.1\r\n";
    if buf.starts_with(get) {
        println!("success get");
    }

    // println!("{}", String::from_utf8_lossy(&buf));
    let content = fs::read_to_string("./index.html").unwrap();

    let respond = format!("HTTP/1.1 200 OK \r\n\r\n{}", content);
    stream.write(respond.as_bytes()).unwrap();
    stream.flush().unwrap();

}