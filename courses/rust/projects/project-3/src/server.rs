use crate::engines::KvsEngine;
use crate::Result;
use log::info;
use std::net::{TcpListener, TcpStream};
// rust里面的struct 定义不需要;
pub struct KvsServer<'a> {
    ip_address: &'a str,
    //engine: E,
}

// 好好设想一下, 还有这个impl的写法, 因为是一个generic type, 所以要这么写s
impl<'a> KvsServer<'a> {
    // 关键在设计的时候就要大概design 一些东西;

    // function 也要加上pub
    pub fn new(ip_address: &'a str) -> Self {
        Self {
            ip_address: ip_address,
        }
    }
    // serve
    pub fn serve(&self) -> Result<()> {
        println!("self ip_address is {}", self.ip_address);
        let listener = TcpListener::bind(self.ip_address)?;
        for stream in listener.incoming() {
            handle(stream?);
        }
        Ok(())
    }
}

fn handle(_: TcpStream) {
    // 一个链接
    println!("connected on connection");
}
