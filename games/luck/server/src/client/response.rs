use serde_derive::{Serialize, Deserialize};

pub enum Code {
    PlayFound,
    WaitingForOpponent,
    InvalidUserGameKey
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResponseData {
    usernames(Vec<String>)
}

pub struct CodeDataPair {
    pub code: Code,
    pub data: Option<ResponseData>
}

pub struct Response {
    code: String,
    data: Option<ResponseData>,
    message: String
}

impl Response {
    pub fn new(code: Code, data: Option<ResponseData>) -> Response {
        match code {
            Code::PlayFound => {
                Response {
                    code: "play_found".to_owned(),
                    data: data,
                    message: "Play found".to_owned()
                }
            },
            Code::WaitingForOpponent => {
                Response {
                    code: "waiting_for_opponent".to_owned(),
                    data: data,
                    message: "Waiting for opponent".to_owned()
                }
            },
            Code::InvalidUserGameKey => {
                Response {
                    code: "invalid_user_game_key".to_owned(),
                    data: data,
                    message: "invalid user game key".to_owned()
                }
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
