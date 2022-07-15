use std::process;

use clap::{arg, command, Command};
fn main() {
    // 关于为clap crate 启用依赖的方法

    // matches 顾名思义就是匹配项

    // 首先要设计
    let matches = command!()
        .subcommand(
            // sub command ==> set, rm, get
            Command::new("set")
                .about("set the key and value pair")
                .arg(arg!(<key> "the key"))
                .arg(arg!(<value> "the value")),
        )
        .subcommand(
            Command::new("get")
                .about("get the value of this key")
                .arg(arg!(<key> "the key")),
        )
        .subcommand(
            Command::new("rm")
                .about("remove the value of this key")
                .arg(arg!(<key> "the key")),
        )
        .get_matches();
    // 关键看学习match语法, 因为返回的是一个Option

    // match 要求...覆盖完全所有的pattern, 但有些情况是不可能被cover完全的
    match matches.subcommand() {
        Some(("set", _)) => {
            panic!("unimplemented");
        }
        Some(("get", _)) => {
            panic!("unimplemented");
        }
        Some(("rm", _)) => {
            panic!("unimplemented");
        }
        _ => {}
    }
    // process::
    process::exit(1);
}

// 注意看bin, src下面有一个bin ==> 有一个main

#[test]
fn test_bin() {
    println!("test_bin");
}
