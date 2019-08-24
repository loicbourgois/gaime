use serde_derive::{Serialize, Deserialize};

use crate::user::*;

pub type PlayId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub play_id: PlayId,
    pub usernames: Vec<Username>,
    pub key: String
}
