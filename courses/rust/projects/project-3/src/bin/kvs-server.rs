use clap::{Arg, Command};
use kvs::{KvsEngine, KvsServer, Result};
use log::info;
use log::LevelFilter;

// 还是StructOpt好用的多
// 写法借鉴一下，固定的ip_addr
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
fn main() -> Result<()> {
    // 了解一下builder是什么, filter_level
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut address = DEFAULT_LISTENING_ADDRESS;
    let mut engine = "default";
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
    if let Some(ip_adder) = matches.get_one::<&str>("ip_addr") {
        info!("the listening addr {}", ip_adder);
        address = ip_adder;
    } else {
        info!("the listening addr {}", DEFAULT_LISTENING_ADDRESS);
    }

    if let Some(egi) = matches.get_one::<&str>("engine") {
        info!("the engine is {}", egi);
        engine = egi;
    } else {
        info!("the engine is default");
    }

    run(address)?;

    Ok(())
    // 现在要做的就是accept tcp socket
}

// ok我们应当对于&str, str, String 本质上都有了新的理解....
// String是一个类型，本质是对str slice的一系列指向, &str则是对其对引用，因此是不具备所有权的
fn run(address: &str) -> Result<()> {
    let server = KvsServer::new(address);
    // 大概就是这样的
    server.serve()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn test_ownership() {
        let mut engine = "123";
        let address = "123213";
        let t = address;
        engine = "456";
        println!("{}", address);
    }
}
