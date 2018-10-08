extern crate openssl;

use openssl::pkcs12::Pkcs12;
use openssl::ssl::{SslVersion, SslOptions, SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {

let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("Unable to create SslAcceptorBuilder");
builder.set_private_key_file("D:/PersonalCoding/rust/client-server/key.pem", SslFiletype::PEM).unwrap();
builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/certificate.pem").unwrap();
builder.check_private_key().unwrap();
builder.clear_options(SslOptions::NO_TLSV1_3);
builder.set_min_proto_version(Some(SslVersion::TLS1_3)).unwrap();
let acceptor = Arc::new(builder.build());

let listener = TcpListener::bind("127.0.0.1:8443").unwrap();


for stream in listener.incoming() {
    if let Ok(stream) = stream {
        let acceptor = acceptor.clone();
            thread::spawn(move || {
                let stream = acceptor.accept(stream).expect("Unable to accept incoming connection.");
                handle_client(stream);
            });
    }
    else {
        println!("Could not connect.");
    }
}
}

fn handle_client(mut stream: SslStream<TcpStream>) {
    let message = String::from("Hello client. This is server !");
    let buf = message.into_bytes();
    let written = stream.write(&buf).unwrap_or_default();
    println!("Written = {}", written);
}