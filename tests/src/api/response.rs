use serde_derive::{Serialize, Deserialize};

use crate::api;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    play(api::data::Play),
    findplay(api::data::FindPlay),
    endplay(api::data::EndPlayOk)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub status: i32,
    pub code: String,
    pub data: Option<Data>,
    pub message: String
}
