use std::io::prelude::*;
use std::net::TcpStream;

static SERVER_ADDRESS: &'static str = "127.0.0.1:12345";


pub fn ping() {
    let mut stream = TcpStream::connect(SERVER_ADDRESS).unwrap();

    let _ = stream.write(&[1]); // ignore the Result
    // let _ = stream.read(&mut [0; 128]); // ignore this too
} // the stream is closed here





#[test]
fn test_client() {

}
