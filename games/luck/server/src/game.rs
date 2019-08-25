use crate::types::*;
use crate::client;
use crate::api;

//use std::rc::Rc;
//use std::cell::RefCell;
//use std::env;
use std::sync::mpsc;
use std::collections::HashMap;
//use std::sync::{Mutex, Arc};
//use std::thread;

use rand::Rng;
use reqwest;

pub type GameCommand = String;

pub struct Game {
    //receivers: HashMap<Username, mpsc::Receiver<GameCommand>>,
    //senders: HashMap<Username, ws::Sender>,
    pub users_data: Vec<UserData>,
    pub api_root_url: String,
    pub play_key: Key,
    pub play_id: PlayId,
}

#[derive(Debug)]
pub struct UserData {
    pub receiver: mpsc::Receiver<GameCommand>,
    pub sender: ws::Sender,
    pub username: Username,
    pub user_game_key: Key
}

impl Game {
    pub fn start(&self) {
        println!("users: {:#?}", self.users_data);
        self.end();
    }

    fn api_url(&self, path: &str) -> String {
        format!("{}{}", self.api_root_url, path)
    }

    fn end(&self) {
        let mut rng = rand::thread_rng();
        // Api
        let mut api_users_data = HashMap::new();
        for user_data in self.users_data.iter() {
            // TODO: make sure the sum of ranks is meaningfull
            let rank: Rank = rng.gen_range(0.0, 10.0);
            api_users_data.insert(user_data.username.clone(), api::data::EndPlayUserData {
                username: user_data.username.clone(),
                rank: rank,
                user_game_key: user_data.user_game_key.clone()
            });
        }
        let api_data = api::data::EndPlay {
            play_key: self.play_key.clone(),
            play_id: self.play_id,
            users_data: api_users_data,
        };
        let api_response: api::response::Response = reqwest::Client::new()
            .post(&self.api_url("/endplay"))
            .json(&api_data)
            .send().unwrap().json().unwrap();
        println!("api_response: {:#?}", api_response);
        // Clients
        for user_data in self.users_data.iter() {
            let response = client::response::Response::new(client::response::Code::GameEnded, None).unwrap();
            client::response::Response::send(&user_data.sender, response);
            user_data.sender.close(ws::CloseCode::Normal);
        }
    }
}
