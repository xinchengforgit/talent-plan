use std::io::{Read, Write};
use std::net::TcpStream;
fn main() {
    match TcpStream::connect("localhost:6379") {
        // match里面也可以加mut
        Ok(mut stream) => {
            println!("successful connect to redis server!");

            let msg = b"*1\r\n$4\r\nPING\r\n"; // rust中使用u8的方式
            stream.write_all(msg).unwrap();
            // array
            let mut data = [0_u8; 10];
            // data[..] 实际上是一个切片
            match stream.read(&mut data) {
                // 返回是()也得使用
                Ok(_) => {
                    println!("{}", std::str::from_utf8(&data).unwrap().to_string());
                }

                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        Err(e) => {
            println!("{}", e);
        }
    }
}
