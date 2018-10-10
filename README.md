# client-server
Simple client and server in Rust over OpenSSL 1.1.1

# Setup
Install OpenSSL 1.1.1 and build rust-openssl against that. Steps are [here](https://github.com/sfackler/rust-openssl/tree/5948898e54882c0bedd12d87569eb4dbee5bbca7).

# Build server
```
cd server
cargo build
```

# Build client
```
cd client
cargo build
```

# Run server
```
cd server\target\debug
.\server.exe [ecc | server-ecc | client-ecc | clientcert | verbose]
```

# Run client
```
cd client\target\debug
.\client.exe [ecc | server-ecc | client-ecc | verbose | tls12 | bench]
```

# Options
`ecc` : Use ECDSA certificates for both client and server

`server-ecc` : Use ECDSA certificate for server and RSA certificate for client

`client-ecc` : Use RSA certificate for server and ECDSA certificate for client

By default, uses RSA certificates for both client and server. Note : Specify only one of the above options at a time or none.

`clientcert` : Server should authenticate client with client certificate

`verbose` : Pass messages and some additional logging

`tls12` : Use TLS 1.2 protocol. By default, uses TLS 1.3.

`bench` : Run a small benchmark to measure time taken for client-server connection over 10000 attempts. Note : Do not run with `verbose` option.



