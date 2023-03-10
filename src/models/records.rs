use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Record {
    pub index:i64,
    pub value:Option<f32>
}
impl Record {
    pub fn new(index:i64,value:Option<f32>) -> Self {
        Self {
            index,
            value
        }
    }
}

impl Default for Record {
    fn default() -> Self {
        Self {
            index:-1,
            value:None
        }
    }
}