use std::collections::HashMap;
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
pub struct KvStore {
    mp: HashMap<String, String>,
}

impl KvStore {
    // 注意Self大小写不一样是有区别的，小写的self是相当于一个crate
    pub fn new() -> Self {
        KvStore { mp: HashMap::new() }
    }
    // 神必的 self 和 Self
    pub fn set(&mut self, key: String, val: String) {
        self.mp.insert(key, val);
    }

    // 关于Option<&T> to Option<T>的方法, map 是一种, 它可以把Option<U> -> Option<T>
    pub fn get(&self, key: String) -> Option<String> {
        self.mp.get(&key).cloned() // 注意cloned的方法, 专门用来将Option<&T> -> Option<T>
                                   // self.mp.get(&key).map(|x| x.to_owned())
    }

    pub fn remove(&mut self, key: String) {
        self.mp.remove(&key);
    }
}
