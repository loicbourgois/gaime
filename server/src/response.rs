use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data;
use crate::data::Play;
use crate::rating::*;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub enum Code {
    PlayFound,
    WaitingForOpponent,
    InvalidUserGameKey,
    EndPlayOk,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseData {
    play(Play),
    findplay(data::FindPlay),
    endplay(data::EndPlayOk),
}

pub struct CodeDataPair {
    pub code: Code,
    pub data: Option<ResponseData>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: ResponseStatus,
    code: String,
    data: Option<ResponseData>,
    message: String,
}

impl Response {
    pub fn new(code: Code, data: Option<ResponseData>) -> Response {
        let status = 200;
        match (code, data.as_ref()) {
            (Code::PlayFound, Some(ResponseData::play(_))) => Response {
                status: status,
                code: "play_found".to_owned(),
                data: data,
                message: "Play found".to_owned(),
            },
            (Code::EndPlayOk, Some(ResponseData::endplay(_))) => Response {
                status: status,
                code: "end_play".to_owned(),
                data: data,
                message: "End play".to_owned(),
            },
            (Code::WaitingForOpponent, None) => Response {
                status: status,
                code: "waiting_for_opponent".to_owned(),
                data: data,
                message: "Waiting for opponent".to_owned(),
            },
            (Code::InvalidUserGameKey, Some(ResponseData::findplay(_))) => Response {
                status: status,
                code: "invalid_user_game_key".to_owned(),
                data: data,
                message: "Invalid user game key".to_owned(),
            },
            _ => {
                panic!("Impossible pair: {:#?} {:#?}", code, data);
            }
        }
    }

    /*pub fn code(&self) -> String {
        self.code.clone()
    }*/

    /*pub fn data(&self) -> Option<ResponseData> {
        self.data.clone()
    }*/

    /*pub fn message(&self) -> String {
        self.message.clone()
    }*/
}
