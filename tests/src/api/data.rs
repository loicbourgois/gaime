use crate::types::*;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub play_id: PlayId,
    pub usernames: Vec<Username>,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindPlay {
    pub username: String,
    pub game_string_id: String,
    pub game_key: String,
    pub user_game_key: String,
}

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
    pub user_game_key: Key,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndPlayOk {
    pub game_id: GameId,
    pub play_id: PlayId,
    pub ratings: HashMap<String, Rating>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyGlicko2Rating {
    pub value: f64,
    pub deviation: f64,
    pub volatility: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub old_rating: MyGlicko2Rating,
    pub new_rating: MyGlicko2Rating,
}
