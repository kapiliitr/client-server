extern crate openssl;
use openssl::ssl::{SslOptions, SslVersion, SslMethod, SslConnector, SslStream, SslFiletype};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs::File;
use std::time::{Instant};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();

builder.set_private_key_file("D:/PersonalCoding/rust/client-server/client/key.pem", SslFiletype::PEM).unwrap();
builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/client/certificate.pem").unwrap();
builder.set_ca_file("D:/PersonalCoding/rust/client-server/server/certificate.pem").unwrap();

if args.contains(&String::from("ecc"))
{
    builder.set_private_key_file("D:/PersonalCoding/rust/client-server/client/eckey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/client/eccertificate.pem").unwrap();
    builder.set_ca_file("D:/PersonalCoding/rust/client-server/server/eccertificate.pem").unwrap();
}
else if args.contains(&String::from("server-ecc"))
{
    builder.set_ca_file("D:/PersonalCoding/rust/client-server/server/eccertificate.pem").unwrap();
}
else if args.contains(&String::from("client-ecc"))
{
    builder.set_private_key_file("D:/PersonalCoding/rust/client-server/client/eckey.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("D:/PersonalCoding/rust/client-server/client/eccertificate.pem").unwrap();
}

if args.contains(&String::from("tls12"))
{
    builder.set_max_proto_version(Some(SslVersion::TLS1_2)).unwrap();
}
else 
{
    builder.clear_options(SslOptions::NO_TLSV1_3);
    builder.set_min_proto_version(Some(SslVersion::TLS1_3)).unwrap();
}

//builder.set_ciphersuites("TLS_CHACHA20_POLY1305_SHA256").unwrap();

let file = File::create("D:/PersonalCoding/rust/client-server/client/keyfile.log").unwrap();
builder.set_keylog_callback(move |_, line| {
    let _ = writeln!(&file, "{}", line);
});

let connector = builder.build();

let config = connector.configure().unwrap();
let version = config.version_str();
println!("openssl version = {}", version);

// Setup

let mut iters = 1;
if args.contains(&String::from("bench"))
{
    iters = 10000;
}

// Benchmark
let start = Instant::now();
// Execute the routine "iters" times
for _ in 0..iters {
    // Code to benchmark using the parameter goes here
    let stream = TcpStream::connect("127.0.0.1:8444").unwrap();
    let mut stream = connector.connect("kaagarw", stream).unwrap();
    handle_server(&mut stream, args.contains(&String::from("verbose"))).unwrap();
}
let elapsed = start.elapsed();

// Teardown

// Report elapsed time in nanoseconds to stdout
println!("{}s {}ns", elapsed.as_secs(), elapsed.subsec_nanos());

}

fn handle_server(stream: &mut SslStream<TcpStream>, verbose: bool) -> Result<(), std::io::Error> {
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();

    if verbose
    {
        println!("{}", stream.get_ref().peer_addr()?);
        println!("{}", String::from_utf8_lossy(&res));
        println!("{}", stream.ssl().version_str());
        if let Some(cipher) = stream.ssl().current_cipher() {
            println!("{}", cipher.description());
        }
    }

Ok(())
}