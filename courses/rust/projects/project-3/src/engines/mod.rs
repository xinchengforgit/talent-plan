use crate::Result;
pub trait KvsEngine {
    // 好好思考一下KvsEngine需要什么, 还有使用这样的写法;
    fn get(&mut self, key: String) -> Result<Option<String>>;

    fn set(&mut self, key: String, val: String) -> Result<()>;

    fn remove(&mut self, key: String) -> Result<()>;
}
