//use crate::client::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub code: String,
    pub data: crate::client::data::Data,
}
