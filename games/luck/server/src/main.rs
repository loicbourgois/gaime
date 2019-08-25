mod api;
mod client;
mod client_error;
mod error;
mod game;
mod types;

use client::response::*;
use client_error::*;
use error::*;
use game::*;
use types::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use std::sync::mpsc;
//use std::sync::{Mutex, Arc};
use std::thread;

use failure::Error;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use ws::{CloseCode, Handshake, Message, Sender};

#[macro_use]
extern crate failure;

struct Handler<'a> {
    sender: Sender,
    //senders: Arc<Mutex<HashMap<Username, Sender>>>,
    senders: Rc<RefCell<HashMap<Username, Sender>>>,
    user_game_keys: Rc<RefCell<HashMap<Username, Key>>>,
    transmitters: Rc<RefCell<HashMap<Username, mpsc::Sender<GameCommand>>>>,
    game_key: String,
    game_string_id: String,
    api_root_url: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse<'a> {
    status: i32,
    code: &'a str,
    message: &'a str,
    data: Option<ResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Play {
    play_id: i32,
    usernames: Vec<String>,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WaitingForOpponent {
    code: String,
    message: String,
}

impl Handler<'_> {
    fn register_user(&self, username: Username, user_game_key: Key) -> Result<(), Error> {
        self.senders
            .borrow_mut()
            .insert(username.clone(), self.sender.clone());
        self.user_game_keys
            .borrow_mut()
            .insert(username.clone(), user_game_key);
        Ok(())
    }

    fn findplay(&self, client_data: client::data::FindPlay) -> Result<Response, Error> {
        let api_data = api::data::FindPlay {
            username: client_data.username.clone(),
            game_string_id: self.game_string_id.clone(),
            game_key: self.game_key.clone(),
            user_game_key: client_data.user_game_key.clone(),
        };
        let api_response: api::response::Response = Client::new()
            .post(&self.api_url("/findplay"))
            .json(&api_data)
            .send()?
            .json()?;
        match api_response.code.as_ref() {
            "play_found" => {
                self.register_user(client_data.username, client_data.user_game_key)?;
                let play_data = match api_response.data.unwrap() {
                    api::response::Data::play(data) => Ok(data),
                    _ => Err(MyError::InvalidPlay {}),
                }?;
                let mut users_data = Vec::new();
                for username in play_data.usernames.iter() {
                    let (tx, rx) = mpsc::channel();
                    self.transmitters.borrow_mut().insert(username.clone(), tx);
                    users_data.push(UserData {
                        receiver: rx,
                        sender: self
                            .senders
                            .borrow_mut()
                            .get(&username.clone())
                            .unwrap()
                            .clone(),
                        username: username.clone(),
                        user_game_key: self
                            .user_game_keys
                            .borrow_mut()
                            .get(&username.clone())
                            .unwrap()
                            .clone(),
                    });
                }
                let api_root_url: String = self.api_root_url.to_string().clone();
                let play_key = play_data.key.clone();
                let play_id = play_data.play_id.clone();
                thread::spawn(move || {
                    let game = Game {
                        users_data: users_data,
                        api_root_url: api_root_url,
                        play_key: play_key,
                        play_id: play_id,
                    };
                    game.start();
                });
                Ok(Response::new(
                    client::response::Code::PlayFound,
                    Some(ResponseData::usernames(play_data.usernames.clone())),
                )?)
            }
            "waiting_for_opponent" => {
                self.register_user(client_data.username, client_data.user_game_key)?;
                Ok(Response::new(
                    client::response::Code::WaitingForOpponent,
                    None,
                )?)
            }
            "invalid_user_game_key" => Ok(Response::new(
                client::response::Code::InvalidUserGameKey,
                None,
            )?),
            any => Err(MyError::UnknownApiCode {
                code: any.to_owned(),
            })?,
        }
    }

    fn send_client_error(&self, error_code: ClientErrorCode) -> Result<(), ws::Error> {
        let error = ClientError::new(error_code);
        let json_response = JsonResponse {
            status: 400,
            code: &error.code(),
            message: &error.message(),
            data: None,
        };
        let text = serde_json::to_string(&json_response).unwrap();
        self.sender.send(Message::text(text))
    }

    fn send_internal_error(&self, error: String) -> Result<(), ws::Error> {
        let json_response = JsonResponse {
            status: 500,
            code: "e500",
            message: "Internal error",
            data: None,
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
                    Ok(request) => request,
                    Err(error) => {
                        return self.send_client_error(ClientErrorCode::CouldNotParseRequest);
                    }
                };
                // Dispatch
                match (request.code.as_ref(), request.data) {
                    ("findplay", client::data::Data::findplay(data)) => match self.findplay(data) {
                        Ok(response) => Response::send(&self.sender, response),
                        Err(error) => self.send_internal_error(error.to_string()),
                    },
                    _ => self.send_client_error(ClientErrorCode::InvalidInputCode),
                }
            }
            Message::Binary(_) => self.send_client_error(ClientErrorCode::BinaryNotAllowed),
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
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
    //let senders = Arc::new(Mutex::new(HashMap::new()));
    let senders = Rc::new(RefCell::new(HashMap::new()));
    let transmitters = Rc::new(RefCell::new(HashMap::new()));
    let user_game_keys = Rc::new(RefCell::new(HashMap::new()));
    let url = env_var_or_default("GAME_LUCK_URL", "0.0.0.0:8080");
    let api_root_url = env_var_or_default("API_ROOT_URL", "http://localhost:8000");
    let game_key = env_var_or_default(
        "GAME_LUCK_KEY",
        "$2y$12$WMofxOYcosOVtiTI4TXjN.qC08VJk3bURb2gDRGO3kr1ZZMPUZxv6",
    );
    let game_string_id = env_var_or_default("GAME_LUCK_STRING_ID", "luck");
    ws::listen(url, |sender| Handler {
        sender: sender,
        senders: Rc::clone(&senders),
        user_game_keys: Rc::clone(&user_game_keys),
        transmitters: Rc::clone(&transmitters),
        game_key: game_key.to_owned(),
        game_string_id: game_string_id.to_owned(),
        api_root_url: &api_root_url,
    })
}
