use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    // 注意Self大小写不一样是有区别的，小写的self是相当于一个crate
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, val: String) {
        self.map.insert(key, val);
    }

    // 关于Option<&T> to Option<T>的方法, map 是一种, 它可以把Option<U> -> Option<T>
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned() // 注意cloned的方法, 专门用来将Option<&T> -> Option<T>
                                    // self.mp.get(&key).map(|x| x.to_owned())
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
