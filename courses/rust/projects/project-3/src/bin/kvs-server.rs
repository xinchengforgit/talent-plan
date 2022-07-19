use clap::{arg, Arg, Command};
use std::process;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("kvs server")
        .arg(
            Arg::new("ip-port")
                .help("the ip address the server will run")
                .long("ip_addr")
                .takes_value(true),
        )
        .arg(
            Arg::new("engine")
                .help("the engine the server will choose")
                .long("engine")
                .takes_value(true),
        )
        .get_matches();
    panic!("unimplemented");
}
