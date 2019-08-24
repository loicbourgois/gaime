use serde_derive::{Serialize, Deserialize};

use crate::api;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: i32,
    pub code: String,
    pub data: Option<api::data::Data>,
    pub message: String
}
