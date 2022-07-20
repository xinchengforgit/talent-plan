use std::io::Error;
// 可以用作re-export
pub use engines::KvsEngine;
mod engines; // 必须要是engines.rs 或者engines/mod.rs
mod server;
pub use server::KvsServer;
pub type Result<T> = std::result::Result<T, Error>;
