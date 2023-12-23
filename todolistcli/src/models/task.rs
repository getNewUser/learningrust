use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Task {
    pub(crate) id:i32,
    pub(crate) name: String,
    pub(crate) is_done: i32
}