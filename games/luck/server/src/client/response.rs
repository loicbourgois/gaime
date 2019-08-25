use crate::error::MyError;
use crate::types::*;

use serde_derive::{Deserialize, Serialize};
use ws;

pub enum Code {
    PlayFound,
    WaitingForOpponent,
    InvalidUserGameKey,
    GameEnded,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseData {
    usernames(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    status: Status,
    code: String,
    data: Option<ResponseData>,
    message: String,
}

impl Response {
    pub fn new(code: Code, data: Option<ResponseData>) -> Result<Response, MyError> {
        match (code, data.as_ref()) {
            (Code::PlayFound, Some(ResponseData::usernames(_))) => Ok(Response {
                status: 200,
                code: "play_found".to_owned(),
                data: data,
                message: "Play found".to_owned(),
            }),
            (Code::WaitingForOpponent, None) => Ok(Response {
                status: 200,
                code: "waiting_for_opponent".to_owned(),
                data: data,
                message: "Waiting for opponent".to_owned(),
            }),
            (Code::InvalidUserGameKey, None) => Ok(Response {
                status: 200,
                code: "invalid_user_game_key".to_owned(),
                data: data,
                message: "invalid user game key".to_owned(),
            }),
            (Code::GameEnded, None) => Ok(Response {
                status: 200,
                code: "game_ended".to_owned(),
                data: data,
                message: "game ended".to_owned(),
            }),
            _ => Err(MyError::InvalidCodeDataPair {})?,
        }
    }

    pub fn send(sender: &ws::Sender, response: Response) -> Result<(), ws::Error> {
        let text = serde_json::to_string(&response).unwrap();
        sender.send(ws::Message::text(text))
    }
}
