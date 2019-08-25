use crate::rating;
use crate::types::*;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindPlay {
    pub username: String,
    pub game_string_id: String,
    pub game_key: Key,
    pub user_game_key: Key,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndPlay {
    pub play_key: Key,
    pub play_id: PlayId,
    pub users_data: HashMap<Username, EndPlayUserData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPlayOk {
    pub game_id: GameId,
    pub play_id: PlayId,
    pub ratings: HashMap<String, rating::Rating>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndPlayUserData {
    pub username: Username,
    pub rank: Rank,
    pub user_game_key: Key,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub play_id: PlayId,
    pub usernames: Vec<Username>,
    pub key: String,
}
