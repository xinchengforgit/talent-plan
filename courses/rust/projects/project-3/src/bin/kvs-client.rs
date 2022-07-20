use clap::{arg, Arg, Command};
use std::net::TcpStream;
use std::process;
fn main() {
    // 关于为clap crate 启用依赖的方法

    // matches 顾名思义就是匹配项

    // 首先要设计
    // 关于command!的本质
    // 要不就用这个吧, 有空再把这个改成structOpt的方式
    // 关于into()函数, 实际上就是把一个type转化为xxx
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("kvs-client")
        .subcommand(
            // sub command ==> set, rm, get
            // 要启动env必须要在crate的feature中启动
            Command::new("set")
                .about("set the key and value pair")
                .arg(arg!(<key> "the key"))
                .arg(arg!(<value> "the value"))
                .arg(
                    Arg::new("ip-port")
                        .long("ip_addr")
                        .takes_value(true)
                        .help("set the ip address"),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("get the value of this key")
                .arg(arg!(<key> "the key"))
                .arg(
                    Arg::new("ip-port")
                        .long("ip_addr")
                        .takes_value(true)
                        .help("set the ip address"),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("remove the value of this key")
                .arg(arg!(<key> "the key"))
                // 关键在于不能使用- hypens连字符
                .arg(
                    Arg::new("ip-port")
                        .long("ip_addr")
                        .takes_value(true)
                        .help("set the ip address"),
                ),
        )
        .get_matches();
    // 关键看学习match语法, 因为返回的是一个Option

    // match 要求...覆盖完全所有的pattern, 但有些情况是不可能被cover完全的
    match matches.subcommand() {
        Some(("set", _)) => {
            if let Ok(_) = TcpStream::connect("127.0.0.1:4000") {
                println!("hello, server");
            }
        }
        Some(("get", _)) => {
            if let Ok(_) = TcpStream::connect("127.0.0.1:4000") {
                println!("hello, server");
            }
        }
        Some(("rm", _)) => {
            if let Ok(_) = TcpStream::connect("127.0.0.1:4000") {
                println!("hello, server");
            }
        }
        _ => {}
    }
    if let Ok(_) = TcpStream::connect("127.0.0.1:4000") {
        println!("hello, server");
    }
    // process::
    process::exit(1);
}
