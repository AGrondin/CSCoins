
extern crate crypto;
extern crate openssl;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

//Everything to do with communicating with the server.
mod server_comms;

fn main() {
    server_comms::CSCoinClient::create_rsa_keys();
    server_comms::CSCoinClient::load_rsa_keys();
    println!("{0:.5}", 2.99999999);
    println!("Hello, world!");
}
