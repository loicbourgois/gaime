use crate::types::*;

use serde_derive::{Serialize, Deserialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    play(Play),
    findplay(FindPlay)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub play_id: PlayId,
    pub usernames: Vec<Username>,
    pub key: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindPlay {
    pub username: String,
    pub game_string_id: String,
    pub game_key: String,
    pub user_game_key: String
}
