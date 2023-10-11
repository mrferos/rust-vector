extern crate ingest;

use ingest::config;
use ingest::server;

fn main() {
    let conf = &config::Settings::new().expect("there was an error loading configuration");
    let bind_addr = &format!("{}:{}", conf.udp().host(), conf.udp().port());
    let s = server::Server::bind(
        bind_addr,
        conf.udp().byte_size()
    );

    println!("{}", bind_addr);
    s.start()
}