use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use ws::{Sender, Message, Handshake, CloseCode};
use serde_json;
use serde_derive::{Serialize, Deserialize};
use reqwest::{Client};
use std::env;

mod client_error;
mod api;
mod client;
mod types;
use client_error::*;
use client::response::*;
use types::*;

#[macro_use] extern crate failure;

use failure::Error;
#[derive(Debug, Fail)]
enum MyError {
    #[fail(display = "invalid play")]
    InvalidPlay {
    },
    #[fail(display = "unkknown api code : {}", code)]
    UnknownApiCode {
        code: String
    }
}


struct Handler<'a> {
    sender: Sender,
    senders: Arc<Mutex<HashMap<Username, Sender>>>,
    game_key: String,
    game_string_id: String,
    api_root_url: &'a str,
}



#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse<'a> {
    status: i32,
    code: &'a str,
    message: &'a str,
    data: Option<ResponseData>
}



#[derive(Serialize, Deserialize, Debug)]
struct Play {
    play_id: i32,
    usernames: Vec<String>,
    key: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WaitingForOpponent {
    code: String,
    message: String
}

impl Handler<'_> {
    fn findplay(&self, client_data: client::data::FindPlay) -> Result<CodeDataPair, Error> {
        let api_data = api::data::FindPlay {
            username: client_data.username,
            game_string_id: self.game_string_id.clone(),
            game_key: self.game_key.clone(),
            user_game_key: client_data.user_game_key
        };
        let api_response: api::response::Response = Client::new()
            .post(&self.api_url("/findplay"))
            .json(&api_data)
            .send()?.json()?;
        match api_response.code.as_ref() {
            "play_found" => {
                let play = match api_response.data.unwrap() {
                    api::data::Data::play(play) => Ok(play),
                    _ => Err(MyError::InvalidPlay{}) 
                }?;
                let usernames = play.usernames;
                
                /*for username in usernames {
                    Handler::send_response(
                        self.senders.get(username),
                        response::Code::PlayFound,
                        Some(ResponseData::usernames( usernames )));
                }*/
                
                // TODO : launch game thread
                
                Ok (CodeDataPair {
                    code: client::response::Code::PlayFound,
                    data: Some(ResponseData::usernames( usernames ))
                } )
            },
            "waiting_for_opponent" => {
                Ok (CodeDataPair {
                    code: client::response::Code::WaitingForOpponent,
                    data: None
                } )
            },
            "invalid_user_game_key" => {
                Ok (CodeDataPair {
                    code: client::response::Code::InvalidUserGameKey,
                    data: None
                } )
            },
            any => {
                Err(MyError::UnknownApiCode{code:any.to_owned()})?
            }
        }
    }

    fn send_response(sender: & Sender, response_code: client::response::Code, response_data: Option<ResponseData>) -> Result<(), ws::Error> {
        let response = Response::new(response_code, response_data);
        let json_response = JsonResponse {
            status: 200,
            code: &response.code(),
            message: &response.message(),
            data: response.data()
        };
        let text = serde_json::to_string(&json_response).unwrap();
        sender.send(Message::text(text))
    }

    fn send_client_error(&self, error_code: ClientErrorCode) -> Result<(), ws::Error> {
        let error = ClientError::new(error_code);
        let json_response = JsonResponse {
            status: 400,
            code: &error.code(),
            message: &error.message(),
            data: None
        };
        let text = serde_json::to_string(&json_response).unwrap();
        self.sender.send(Message::text(text))
    }

    fn send_internal_error(&self, error: String) -> Result<(), ws::Error> {
        let json_response = JsonResponse {
            status: 500,
            code: "e500",
            message: "Internal error",
            data: None
        };
        println!("[ERROR] {}", error);
        let text = serde_json::to_string(&json_response).unwrap();
        self.sender.send(Message::text(text))
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}{}", self.api_root_url, path)
    }
}

impl ws::Handler for Handler<'_> {

    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<(), ws::Error> {
        match message {
            Message::Text(text) => {
                let request: crate::client::request::Request = match serde_json::from_str(&text) {
                    Ok(request) => {
                        request
                    },
                    Err(error) => {
                        return self.send_client_error(ClientErrorCode::CouldNotParseRequest);
                    }
                };
                // Dispatch
                match (request.code.as_ref(), request.data) {
                    ("findplay",  client::data::Data::findplay(data) ) => {
                        match self.findplay(data) {
                            Ok(code_data_pair) => {
                                Handler::send_response(&self.sender, code_data_pair.code, code_data_pair.data)
                            },
                            Err(error) => {
                                self.send_internal_error(error.to_string())
                            }
                        }
                    },
                    _ => {
                        self.send_client_error(ClientErrorCode::InvalidInputCode)
                    }
                }
            },
            Message::Binary(_) => {
                self.send_client_error(ClientErrorCode::BinaryNotAllowed)
            }
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }

}

fn env_var_or_default(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) => value,
        Err(error) => {
            println!("[WARN] Environnement variable {} does not exist.", key);
            println!("[WARN]\tUsing {} instead.", default_value);
            default_value.to_owned()
        }
    }
}

fn main() -> Result<(), ws::Error> {
    let senders = Arc::new(Mutex::new(HashMap::new()));
    let url = env_var_or_default("GAME_LUCK_URL", "0.0.0.0:8080");
    let api_root_url = env_var_or_default("API_ROOT_URL", "http://localhost:8000");
    let game_key = env_var_or_default("GAME_LUCK_KEY", "$2y$12$WMofxOYcosOVtiTI4TXjN.qC08VJk3bURb2gDRGO3kr1ZZMPUZxv6");
    let game_string_id = env_var_or_default("GAME_LUCK_STRING_ID", "luck");
    ws::listen(url, |sender| {
        Handler {
            sender: sender,
            senders: Arc::clone(&senders),
            game_key: game_key.to_owned(),
            game_string_id: game_string_id.to_owned(),
            api_root_url: &api_root_url
        }
    })
}
