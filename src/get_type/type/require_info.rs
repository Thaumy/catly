use std::fmt::{Debug, Formatter};

// 需要类型信息
// 此情况由 namely case 产生时表明缺乏 ref_name 的类型信息
// discard case 产生该情况则表明某个弃元值缺乏关键的类型信息
#[derive(PartialEq, Clone)]
pub struct RequireInfo {
    pub ref_name: String
}

impl Debug for RequireInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("ReqInfo::{:?}", self.ref_name))
    }
}
