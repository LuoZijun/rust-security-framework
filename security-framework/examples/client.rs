extern crate security_framework;
use security_framework::secure_transport::ClientBuilder;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("google.com:443").unwrap();

    let mut builder = ClientBuilder::new();
    builder.use_sni(true);

    #[cfg(feature = "OSX_10_14")]
    builder.alpn_protocols(&["h2", "http/1.1"]);

    let mut stream = builder.handshake("google.com", stream).unwrap();
    println!(
        "negotiated chipher: {:?}",
        stream.context().negotiated_cipher().unwrap()
    );
    println!(
        "negotiated version: {:?}",
        stream.context().negotiated_protocol_version().unwrap()
    );

    #[cfg(feature = "OSX_10_14")]
    println!("alpn: {:?}", stream.context().alpn_protocols() );

    stream.write_all(b"GET / HTTP/1.1\r\n\r\n").unwrap();
    stream.flush().unwrap();

    let mut buf = vec![0u8; 1024];
    let amt = stream.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..amt]));
}
