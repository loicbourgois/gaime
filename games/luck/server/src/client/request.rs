//use crate::client::*;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub code: String,
    pub data: crate::client::data::Data
}
