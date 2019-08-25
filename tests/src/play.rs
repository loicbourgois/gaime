use serde_derive::{Deserialize, Serialize};

use crate::user::*;

pub type PlayId = i32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Play {
    pub play_id: PlayId,
    pub usernames: Vec<Username>,
    pub key: String,
}
