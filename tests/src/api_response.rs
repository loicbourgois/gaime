use serde_derive::{Deserialize, Serialize};

use crate::play::*;

#[derive(Debug, Clone, Copy)]
pub enum Code {
    PlayFound,
    WaitingForOpponent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseData {
    //usernames(Vec<String>),
    play(Play),
    enplay(),
}

pub struct CodeDataPair {
    pub code: Code,
    pub data: Option<ResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    status: i32,
    code: String,
    pub data: Option<ResponseData>,
    pub message: String,
}
