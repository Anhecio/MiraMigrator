// zh_CN
pub mod zh_cn;
// en_US
pub mod en_us;

pub trait Interface {
    fn init(&self);
    fn start(&self);
    fn exit(&self);
}
