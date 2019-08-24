use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::data::Play;
use crate::types::*;
use crate::rating::*;
use crate::data;

#[derive(Serialize, Deserialize)]
pub struct EndPlay {
    pub game_id: GameId,
    pub play_id: PlayId,
    pub ratings: HashMap<String, Rating>
}

#[derive(Debug, Clone, Copy)]
pub enum Code {
    PlayFound,
    WaitingForOpponent,
    InvalidUserGameKey
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseData {
    play(Play),
    findplay(data::FindPlay)
}

pub struct CodeDataPair {
    pub code: Code,
    pub data: Option<ResponseData>
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: ResponseStatus,
    code: String,
    data: Option<ResponseData>,
    message: String
}

impl Response {
    pub fn new(code: Code, data: Option<ResponseData>) -> Response {
        let status = 200;
        match (code, data.clone()) {
            (Code::PlayFound, Some(ResponseData::play(_))) => {
                Response {
                    status: status,
                    code: "play_found".to_owned(),
                    data: data,
                    message: "Play found".to_owned()
                }
            },
            (Code::WaitingForOpponent, None) => {
                Response {
                    status: status,
                    code: "waiting_for_opponent".to_owned(),
                    data: data,
                    message: "Waiting for opponent".to_owned()
                }
            },
            (Code::InvalidUserGameKey, Some(ResponseData::findplay(_))) => {
                Response {
                    status: status,
                    code: "invalid_user_game_key".to_owned(),
                    data: data,
                    message: "Invalid user game key".to_owned()
                }
            },
            _ => {
                panic!("Impossible pair: {:#?} {:#?}", code, data);
            }
        }
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn data(&self) -> Option<ResponseData> {
        self.data.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}
