use clap::{Arg, Command};
use log::LevelFilter;
use log::{error, info, warn};
use std::process;

// 写法借鉴一下，固定的ip_addr
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
fn main() {
    // 了解一下builder是什么, filter_level
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("kvs server")
        .arg(
            // new create a unique name
            Arg::new("ip_addr")
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
    info!(
        "starting the server, the current version {}, ",
        env!("CARGO_PKG_VERSION")
    );
    if let Some(ip_adder) = matches.get_one::<String>("ip_addr") {
        info!("the listening addr {}", ip_adder);
    } else {
        info!("the listening addr {}", DEFAULT_LISTENING_ADDRESS);
    }

    if let Some(engine) = matches.get_one::<String>("engine") {
        info!("the engine is {}", engine);
    } else {
        info!("the engine is default");
    }
    // 相当于是我现在可以接受到这些参数了，然后可以获取东西
    panic!("unimplemented");
}
