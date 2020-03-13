extern crate security_framework;

use security_framework::secure_transport::ClientBuilder;
use security_framework::secure_transport::ClientHandshakeError;

use std::io::{self, Read, Write};
use std::net::TcpStream;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = "video.qq.com";
    let port   = 443u16;
    
    let stream = TcpStream::connect(format!("{}:{}", domain, port))?;
    
    let mut builder = ClientBuilder::new();
    builder.use_sni(true);

    #[cfg(feature = "OSX_10_14")]
    builder.alpn_protocols(&["h2", "http/1.1"]);

    let mut ssl_stream = builder.handshake(domain, stream)
        .map_err(|handshake_error| {
            match handshake_error {
                ClientHandshakeError::Failure(e) => io::Error::new(io::ErrorKind::Other, e),
                ClientHandshakeError::Interrupted(_) => {
                    io::Error::new(io::ErrorKind::WouldBlock, "The handshake was interrupted midway through.")
                }
            }
        })?;
    
    let ssl_context = ssl_stream.context();
    println!("negotiated chipher: {:?}", ssl_context.negotiated_cipher()?);
    println!("negotiated version: {:?}", ssl_context.negotiated_protocol_version()?);
    #[cfg(feature = "OSX_10_14")]
    println!("negotiated alpns: {:?}", ssl_context.alpn_protocols()?);

    ssl_stream.write_all(b"GET / HTTP/1.1\r\n\r\n")?;
    ssl_stream.flush()?;

    let mut buf = vec![0u8; 1024];
    let amt = ssl_stream.read(&mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf[..amt]));

    Ok(())
}
