extern crate openssl;
use openssl::ssl::{SslOptions, SslVersion, SslMethod, SslConnector, SslStream};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

builder.set_ca_file("D:/PersonalCoding/rust/client-server/certificate.pem").unwrap();
builder.clear_options(SslOptions::NO_TLSV1_3);
builder.set_min_proto_version(Some(SslVersion::TLS1_3)).unwrap();
let connector = builder.build();

let config = connector.configure().unwrap();
let version = config.version_str();
println!("openssl version = {}", version);

// let alpn = config.selected_alpn_protocol().unwrap();
// println!("alpn = {:?}", alpn);


//let stream = TcpStream::connect("google.com:443").unwrap();
//let mut stream = connector.connect("google.com", stream).unwrap();

let stream = TcpStream::connect("127.0.0.1:8443").unwrap();
let mut stream = connector.connect("kaagarw", stream).unwrap();

//stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
handle_server(&mut stream).unwrap();
}

fn handle_server(stream: &mut SslStream<TcpStream>) -> Result<(), std::io::Error> {
    println!("{}", stream.get_ref().peer_addr()?);
let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));
Ok(())
}