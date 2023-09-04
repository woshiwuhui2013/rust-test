mod ToolUtil;

// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::fs;
// use std::thread;
fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //
    // for _stream in listener.incoming() {
    //     let stream = _stream.unwrap();
    //     println!("Connection established!");
    //
    //     thread::spawn(||{handleStream(stream)});
    //
    // }
    let a = ToolUtil::Info::new();
    a.display()

}

// fn handleStream(mut stream: TcpStream){
//
//     let mut buf  = [0;512];
//
//     stream.read(&mut buf);
//     println!("{}", String::from_utf8_lossy(&buf));
//
//     let get = b"GET / HTTP/1.1\r\n";
//     let (status, filename) = if buf.starts_with(get) {
//         println!("success get");
//         ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
//     }else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
//     };
//
//     println!("{}{}", filename, status );
//
//     // println!("{}", String::from_utf8_lossy(&buf));
//     let content = fs::read_to_string(filename).unwrap();
//     println!("file content {}", content);
//
//     let respond = format!("{}Content-Type: text/html; charset=utf-8\r\n{}", status, content);
//     println!("rsp {}", respond);
//     stream.write(respond.as_bytes()).unwrap();
//     stream.flush().unwrap();
//
// }