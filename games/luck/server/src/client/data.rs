use crate::types::*;

use serde_derive::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    username(String),
    findplay(FindPlay),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindPlay {
    pub username: Username,
    pub user_game_key: Key,
}
