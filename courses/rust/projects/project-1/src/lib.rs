use core::panic;

//lib.rs是相当于package的root crate
#[warn(non_snake_case)]
// pub mod KvStore {
//     pub fn new() {
//         panic!("new unimplemented");
//     }
//     pub fn set(key: String, val: String) {
//         panic!("set unimplemented");
//     }
//     pub fn get(key: String) -> Option<String> {
//         panic!("get unimplemented");
//     }
//     pub fn remove(key: String) {
//         panic!("remove unimplemented");
//     }
// }
pub struct KvStore {}

impl KvStore {
    // 注意Self大小写不一样是有区别的，小写的self是相当于一个crate
    pub fn new() -> Self {
        KvStore {}
    }
    // 神必的 self 和 Self
    pub fn set(&mut self, key: String, val: String) {
        panic!("{}, {}", key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!("{}", key);
    }

    pub fn remove(&self, key: String) {
        panic!("{}", key);
    }
}
