use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
 

pub fn start(addr : &str) {
    let server: SocketAddr = addr.parse().expect("Unable to parse socket address");
    let listener = TcpListener::bind(server).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
 
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
 
fn handle_client(mut stream: TcpStream) {
    println!("getit");
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        println!("{}", String::from_utf8_lossy(&buffer));
        let response = format!("Hello, {}. Your message is : {}", stream.peer_addr().unwrap().to_string(), String::from_utf8_lossy(&buffer));
        stream.write(response.as_bytes()).unwrap();
        buffer = [0; 512];
    }
}
 
pub fn client(addr: &str) {
    let server: SocketAddr = addr.parse().expect("Unable to parse socket address");
    let mut stream = TcpStream::connect(server).unwrap();
    println!("successful connect");
    let mut message = String::new();
    loop{
        println!("print message");
        std::io::stdin().read_line(&mut message).unwrap();
        stream.write(message.as_bytes()).unwrap();
        
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
        message = "".to_string();
    }
}