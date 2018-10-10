extern crate openssl;

use openssl::ssl::{SslVersion, SslOptions, SslMethod, SslAcceptor, SslStream, SslFiletype, SslVerifyMode};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::{thread, env};
use std::io::Write;

fn main() {
let args: Vec<String> = env::args().collect();

let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("Unable to create SslAcceptorBuilder");

builder.set_private_key_file("D:/PersonalCoding/rust/client-server/server/key.pem", SslFiletype::PEM).unwrap();
builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/server/certificate.pem").unwrap();
builder.set_ca_file("D:/PersonalCoding/rust/client-server/client/certificate.pem").unwrap();
if args.contains(&String::from("ecc"))
{
    builder.set_private_key_file("D:/PersonalCoding/rust/client-server/server/eckey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/server/eccertificate.pem").unwrap();
    builder.set_ca_file("D:/PersonalCoding/rust/client-server/client/eccertificate.pem").unwrap();
}
else if args.contains(&String::from("server-ecc"))
{
    builder.set_private_key_file("D:/PersonalCoding/rust/client-server/server/eckey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/server/eccertificate.pem").unwrap();
}
else if args.contains(&String::from("client-ecc"))
{
    builder.set_ca_file("D:/PersonalCoding/rust/client-server/client/eccertificate.pem").unwrap();
}

builder.check_private_key().unwrap();
builder.clear_options(SslOptions::NO_TLSV1_3);
builder.set_min_proto_version(Some(SslVersion::TLS1_2)).unwrap();

if args.contains(&String::from("clientcert"))
{
    let mut verify_mode = SslVerifyMode::empty();
    verify_mode.insert(SslVerifyMode::PEER);
    verify_mode.insert(SslVerifyMode::FAIL_IF_NO_PEER_CERT);
    builder.set_verify(verify_mode);
}

println!("{:#?}", builder.options());
let acceptor = Arc::new(builder.build());

let listener = TcpListener::bind("127.0.0.1:8444").unwrap();

for stream in listener.incoming() {
    if let Ok(stream) = stream {
        let acceptor = acceptor.clone();
            let verbose = args.contains(&String::from("verbose"));
            thread::spawn(move || {
                let stream = acceptor.accept(stream).expect("Unable to accept incoming connection.");
                handle_client(stream, verbose);
            });
    }
    else {
        println!("Could not connect.");
    }
}
}

fn handle_client(mut stream: SslStream<TcpStream>, verbose: bool) {
    if verbose
    {
        let message = String::from("Hello client. This is server !");
        let buf = message.into_bytes();
        let written = stream.write(&buf).unwrap_or_default();
        println!("Written = {}", written);
    }
}