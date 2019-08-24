use crate::types::*;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndPlay {
    pub play_key: Key,
    pub play_id: PlayId,
    pub users_data: HashMap<Username, EndPlayUserData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndPlayUserData {
    pub username: Username,
    pub rank: Rank,
    pub user_game_key: Key
}


#[derive(Serialize, Deserialize)]
pub struct FindPlay {
    pub username: String,
    pub game_string_id: String,
    pub game_key: String,
    pub user_game_key: String
}
