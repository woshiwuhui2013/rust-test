use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for _stream in listener.incoming() {
        let stream = _stream.unwrap();
        println!("Connection established!");

        handleStream(stream);

    }
}

fn handleStream(mut stream: TcpStream){

    let mut buf  = [0;512];

    stream.read(&mut buf);

    // println!("{}", String::from_utf8_lossy(&buf));
    let respond = "HTTP/1.1 200 OK \r\n\r\n";
    stream.write(respond.as_bytes()).unwrap();
    stream.flush().unwrap();

}