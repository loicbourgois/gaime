extern crate ws;

#[macro_use] extern crate serde_derive;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};
use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};
use std::cell::RefMut;

#[derive(Serialize, Deserialize)]
struct FindGameData {
    user: User,
    game_string_id: String
}

type Username = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: Username,
    jwt: String
}

struct WebSocketHandler {
    sender: Sender,
    user: Option<User>,
    waiting_users: Rc<RefCell<HashMap<Username, User>>>,
    playing_users: Rc<RefCell<HashMap<Username, User>>>,
    senders: Rc<RefCell<HashMap<Username, Sender>>>
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse<'a> {
    status: &'a str,
    error_message: &'a str
}

impl ErrorResponse<'_> {
    fn new<'a>(error_message: &'a str) -> ErrorResponse<'a> {
        ErrorResponse {
            status: "error",
            error_message: error_message
        }
    }

    fn as_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl WebSocketHandler {
    fn send_error(&self, error_message: & str) -> Result<()> {
        println!("Error: {}", error_message);
        let error = ErrorResponse::new(error_message);
        self.sender.send(ws::Message::Text(error.as_json_string()))
    }

    fn find_game(&mut self, data: String) -> Result<()> {
        match serde_json::from_str::<FindGameData>(&data) {
            Ok(find_game_data) => {
                let mut playing_users = self.playing_users.borrow_mut();
                match playing_users.get(&find_game_data.user.username) {
                    Some(user) => {
                        self.send_error(&format!("User {} already in game", user.username))
                    },
                    None => {
                        let mut waiting_users = self.waiting_users.borrow_mut();
                        waiting_users.insert(find_game_data.user.username.clone(), find_game_data.user.clone());
                        self.senders.borrow_mut().insert(find_game_data.user.username.clone(), self.sender.clone());
                        self.user = Some(find_game_data.user);
                        match waiting_users.len() {
                            0 => {
                                self.send_error("No waiting_users")
                            },
                            1 => {
                                self.sender.send("Waiting for an opponent")
                            },
                            _ => {
                                self.find_opponent(&mut waiting_users, &mut playing_users)
                            }
                        }
                    }
                }
            },
            Err(error) => {
                self.send_error(&error.to_string())
            }
        }
    }

    fn find_opponent(
        & self,
        waiting_users: &mut RefMut<HashMap<Username, User>>,
        playing_users: &mut RefMut<HashMap<Username, User>>
    ) -> Result<()> {
        match &self.user {
            Some(user) => {
                let player_1_username = &user.username;
                match waiting_users.remove(player_1_username) {
                    Some(player_1) => {
                        match waiting_users.clone().keys().next() {
                            Some(player_2_username) => {
                                match waiting_users.remove(player_2_username) {
                                    Some(player_2) => {
                                        playing_users.insert(player_1_username.to_string(), player_1);
                                        playing_users.insert(player_2_username.to_string(), player_2);
                                        match self.senders.borrow().get(player_2_username) {
                                            Some(player_2_sender) => {
                                                let player_1_sender = &self.sender;
                                                //
                                                // TODO
                                                // spin new thread with
                                                // - player_1_sender
                                                // - player_2_sender
                                                // - player_1
                                                // - player_2
                                                //
                                                player_2_sender.send("Opponent found");
                                                self.sender.send("Opponent found")
                                            },
                                            None => {
                                                self.send_error("No sender for opponent")
                                            }
                                        }
                                    },
                                    None => {
                                        self.send_error("No player 2")
                                    }
                                }
                            },
                            None => {
                                println!("waiting_users : {:#?}", waiting_users);
                                self.send_error("No opponent")
                            }
                        }
                    },
                    None => {
                        self.send_error("No player 1")
                    }
                }
            },
            None => {
                self.send_error("No user")
            }
        }
    }
}

impl Handler for WebSocketHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Received: {}", msg);
        let message = msg.as_text().unwrap();
        let message_json: Value = serde_json::from_str(message).unwrap();
        match message_json["command"].as_str() {
            Some(command) => {
                match command {
                    "findgame" => {
                        self.find_game(message_json["data"].to_string())
                    },
                    command => {
                        self.send_error(&format!("Unknown command: {}", command))
                    }
                }
            },
            None => {
                self.send_error("No command")
            }
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    let waiting_users = Rc::new(RefCell::new(HashMap::new()));
    let playing_users = Rc::new(RefCell::new(HashMap::new()));
    let senders = Rc::new(RefCell::new(HashMap::new()));
    listen(
        "0.0.0.0:8080",
        |sender| WebSocketHandler {
            sender: sender,
            user: None,
            waiting_users: waiting_users.clone(),
            playing_users: playing_users.clone(),
            senders: senders.clone()
        }
    ).unwrap()
}
