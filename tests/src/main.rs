mod api;
mod api_request_data;
mod api_response;
mod play;
mod types;
mod user;

use api_response::*;
use play::*;
use user::*;

use std::collections::HashMap;
use std::io;
use std::process::Command;
use std::process::Output;
use std::thread;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use ws::{CloseCode, Handler, Handshake, Message, Sender};

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    string_id: String,
    name: String,
    description: String,
    websocket_url: String,
    player_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameWithKey {
    string_id: String,
    key: String,
}

#[derive(Serialize)]
struct UserWithPassword<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserWithJwt {
    username: String,
    email: String,
    jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserLogin<'a> {
    username: &'a str,
    password: &'a str,
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

/*#[derive(Serialize, Deserialize, Debug)]
struct EndPlayRequest {
    play_key: String,
    ranking: HashMap<String, f32>,
    play_id: i32
}*/

#[derive(Serialize, Deserialize, Debug)]
struct EndPlay {
    game_id: i32,
    play_id: i32,
    ratings: HashMap<String, Rating>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rating {
    old_rating: MyGlicko2Rating,
    new_rating: MyGlicko2Rating,
}

#[derive(Serialize, Deserialize, Debug)]
struct MyGlicko2Rating {
    value: f64,
    deviation: f64,
    volatility: f64,
}

static PSQL_COMMAND: &str = "psql";
static POSTGRESQL_URL: &str = "postgres://gaimemaster:password@localhost:5432/gaime";
static USER_1: UserWithPassword = UserWithPassword {
    username: "user_1",
    email: "user_1@gaime.org",
    password: "azerty",
};
static USER_2: UserWithPassword = UserWithPassword {
    username: "user_2",
    email: "user_2@gaime.org",
    password: "ploup",
};
static USER_3: UserWithPassword = UserWithPassword {
    username: "user_3",
    email: "user_3@gaime.org",
    password: "qeergqrg",
};

static ADMIN_LOGIN: UserLogin = UserLogin {
    username: "admin",
    password: "123456",
};
static GAME_DESIGNER_LOGIN: UserLogin = UserLogin {
    username: "gamedesigner",
    password: "123456",
};
static USER_1_LOGIN: UserLogin = UserLogin {
    username: "user_1",
    password: "azerty",
};
static USER_2_LOGIN: UserLogin = UserLogin {
    username: "user_2",
    password: "ploup",
};
static USER_3_LOGIN: UserLogin = UserLogin {
    username: "user_3",
    password: "qeergqrg",
};

fn run_command_verbose(command: &str, args: &[&str]) -> Result<Output, io::Error> {
    let output = Command::new(command).args(args).output()?;
    println!("command: {} {:#?}", command, args);
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("");
    assert!(String::from_utf8_lossy(&output.stderr) == "");
    assert!(output.status.success());
    Ok(output)
}

fn api_url(path: &str) -> String {
    const API_ROOT_URL: &str = "http://localhost:8000";
    format!("{}{}", API_ROOT_URL, path)
}

fn game_luck() -> Game {
    Game {
        string_id: "luck".to_owned(),
        name: "Luck".to_owned(),
        description: "Test your luck".to_owned(),
        websocket_url: "ws://0.0.0.0:8080".to_owned(),
        player_count: 3,
    }
}

fn game_snake() -> Game {
    Game {
        string_id: "snake_1v1".to_owned(),
        name: "Snake 1v1".to_owned(),
        description: "The last Snake standing wins".to_owned(),
        websocket_url: "ws://0.0.0.0:8081".to_owned(),
        player_count: 2,
    }
}

#[test]
fn test_001() {
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[POSTGRESQL_URL, "-f", "../database/drop.sql"],
    )
    .unwrap();
    assert!(output.status.success(), true);
}

#[test]
fn test_002() {
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[POSTGRESQL_URL, "-f", "../database/create.sql"],
    )
    .unwrap();
    assert!(output.status.success(), true);
}

#[test]
fn test_003() {
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[POSTGRESQL_URL, "-f", "../database/insert-mock-admins.sql"],
    )
    .unwrap();
    assert!(output.status.success(), true);
}

#[test]
fn test_004() {
    let mut response = reqwest::get(&api_url("/games")).unwrap();
    println!("response: {:#?}", response);
    let games: Vec<Game> = response.json().unwrap();
    println!("games: {:#?}", games);
    assert!(games.len() == 0);
}

#[test]
fn test_005_signup() {
    let mut response = Client::new()
        .post(&api_url("/users"))
        .json(&USER_1)
        .send()
        .unwrap();
    println!("response: {:#?}", response);
    let user_1: UserWithJwt = response.json().unwrap();
    println!("user_1: {:#?}", user_1);
    let user_2: UserWithJwt = Client::new()
        .post(&api_url("/users"))
        .json(&USER_2)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user_2: {:#?}", user_2);
    let user_3: UserWithJwt = Client::new()
        .post(&api_url("/users"))
        .json(&USER_3)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user_3: {:#?}", user_3);
}

#[test]
fn test_006() {
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[POSTGRESQL_URL, "-f", "../database/select.sql"],
    )
    .unwrap();
    assert!(output.status.success(), true);
}

#[test]
fn test_007_login() {
    let mut response = Client::new()
        .post(&api_url("/login"))
        .json(&USER_1_LOGIN)
        .send()
        .unwrap();
    println!("response: {:#?}", response);
    let user: UserWithJwt = response.json().unwrap();
    println!("user: {:#?}", user);
}

#[test]
fn test_008_games_post() {
    let game_designer: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&GAME_DESIGNER_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("game_designer: {:#?}", game_designer);
    let mut response = Client::new()
        .post(&api_url("/games"))
        .json(&game_luck())
        .header("Authorization", game_designer.jwt.clone())
        .send()
        .unwrap();
    println!("response: {:#?}", response);
    let game: Game = response.json().unwrap();
    println!("game: {:#?}", game);
}

// Game creation by an unauthorized user
#[test]
fn test_009_games_post_2() {
    let user: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&USER_1_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user: {:#?}", user);
    let mut response = Client::new()
        .post(&api_url("/games"))
        .json(&game_snake())
        .header("Authorization", user.jwt.clone())
        .send()
        .unwrap();
    println!("response: {:#?}", response);
    println!("response.text: {:#?}", response.text().unwrap());
    //let game: Game = response.json().unwrap();
    //println!("game: {:#?}", game);
}

#[test]
fn test_010_games_post_3() {
    let game_designer: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&GAME_DESIGNER_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("game_designer: {:#?}", game_designer);
    let mut response = Client::new()
        .post(&api_url("/games"))
        .json(&game_snake())
        .header("Authorization", game_designer.jwt.clone())
        .send()
        .unwrap();
    let game: Game = response.json().unwrap();
    println!("game: {:#?}", game);
}

#[test]
fn test_011() {
    let mut response = reqwest::get(&api_url("/games")).unwrap();
    println!("response: {:#?}", response);
    let games: Vec<Game> = response.json().unwrap();
    println!("games: {:#?}", games);
    assert!(games.len() == 2);
}

#[test]
fn test_012() {
    let mut response = reqwest::get(&api_url("/games/luck")).unwrap();
    println!("response: {:#?}", response);
    let game: Game = response.json().unwrap();
    println!("game: {:#?}", game);
}

#[test]
//#[ignore]
fn test_013_game_cycle_from_server() {
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[
            POSTGRESQL_URL,
            "-f",
            "../database/insert-mock-user-game-keys.sql",
        ],
    )
    .unwrap();
    assert!(output.status.success(), true);
    let mock_key = "123456";
    let game: Game = reqwest::get(&api_url("/games/luck"))
        .unwrap()
        .json()
        .unwrap();
    println!("game: {:#?}", game);
    let game_designer: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&GAME_DESIGNER_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("game_designer: {:#?}", game_designer);
    let mut response_1 = Client::new()
        .post(&api_url("/games/luck/newkey"))
        .json(&game_luck())
        .header("Authorization", game_designer.jwt.clone())
        .send()
        .unwrap();
    println!("response_1: {:#?}", response_1);
    let game_with_key: GameWithKey = response_1.json().unwrap();
    println!("game_with_key: {:#?}", game_with_key);
    for i in 0..1 {
        let findplay_response_1: ApiResponse = Client::new()
            .post(&api_url("/findplay"))
            .json(&api_request_data::FindPlay {
                username: USER_1.username.to_string(),
                game_string_id: game_luck().string_id,
                game_key: game_with_key.key.clone(),
                user_game_key: mock_key.to_owned(),
            })
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("findplay_response_1: {:#?}", findplay_response_1);
        let findplay_response_2: ApiResponse = Client::new()
            .post(&api_url("/findplay"))
            .json(&api_request_data::FindPlay {
                username: USER_2.username.to_string(),
                game_string_id: game_luck().string_id,
                game_key: game_with_key.key.clone(),
                user_game_key: mock_key.to_owned(),
            })
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("findplay_response_2: {:#?}", findplay_response_2);
        let find_play_response_3: ApiResponse = Client::new()
            .post(&api_url("/findplay"))
            .json(&api_request_data::FindPlay {
                username: USER_3.username.to_string(),
                game_string_id: game_luck().string_id,
                game_key: game_with_key.key.clone(),
                user_game_key: mock_key.to_owned(),
            })
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("find_play_response_3: {:#?}", find_play_response_3);
        let play = match find_play_response_3.data {
            Some(ResponseData::play(play)) => play,
            _ => panic!(""),
        };
        println!("play: {:#?}", play);
        let mut users_data = HashMap::new();
        users_data.insert(
            USER_1.username.to_string(),
            api_request_data::EndPlayUserData {
                username: USER_1.username.to_string(),
                rank: 2.5,
                user_game_key: mock_key.to_owned(),
            },
        );
        users_data.insert(
            USER_2.username.to_string(),
            api_request_data::EndPlayUserData {
                username: USER_2.username.to_string(),
                rank: 1.0,
                user_game_key: mock_key.to_owned(),
            },
        );
        users_data.insert(
            USER_3.username.to_string(),
            api_request_data::EndPlayUserData {
                username: USER_3.username.to_string(),
                rank: 2.5,
                user_game_key: mock_key.to_owned(),
            },
        );
        let endplay: api::response::Response = Client::new()
            .post(&api_url("/endplay"))
            .json(&api_request_data::EndPlay {
                play_key: play.key,
                play_id: play.play_id,
                users_data: users_data,
            })
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("endplay: {:#?}", endplay);
        let endplay_data = match endplay.data.unwrap() {
            api::response::Data::endplay(v) => v,
            _ => panic!("Wrong response"),
        };
        if i == 0 {
            assert!(
                endplay_data.ratings.get("user_1").unwrap().old_rating.value
                    == endplay_data.ratings.get("user_2").unwrap().old_rating.value
            );
            assert!(
                endplay_data.ratings.get("user_1").unwrap().old_rating.value
                    == endplay_data.ratings.get("user_3").unwrap().old_rating.value
            );
        }
        assert!(
            endplay_data.ratings.get("user_2").unwrap().new_rating.value
                > endplay_data.ratings.get("user_1").unwrap().new_rating.value
        );
        assert!(
            endplay_data.ratings.get("user_1").unwrap().new_rating.value
                == endplay_data.ratings.get("user_3").unwrap().new_rating.value
        );
    }
}

#[test]
//#[ignore]
fn test_014_game_cycle_from_clients() {
    // Mock data
    let output = run_command_verbose(
        PSQL_COMMAND,
        &[POSTGRESQL_URL, "-f", "../database/set-mock-game-keys.sql"],
    )
    .unwrap();
    assert!(output.status.success(), true);
    // Users
    let user_1: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&USER_1_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user_1: {:#?}", user_1);
    let user_2: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&USER_2_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user_2: {:#?}", user_2);
    let user_3: UserWithJwt = Client::new()
        .post(&api_url("/login"))
        .json(&USER_3_LOGIN)
        .send()
        .unwrap()
        .json()
        .unwrap();
    println!("user_3: {:#?}", user_3);
    // Game
    let game: Game = reqwest::get(&api_url("/games/luck"))
        .unwrap()
        .json()
        .unwrap();
    println!("game: {:#?}", game);
    // Keys
    let usergame_with_key_1: GameWithKey = Client::new()
        .post(&api_url("/users/user_1/games/luck/key"))
        .header("Authorization", user_1.jwt.clone())
        .send()
        .unwrap()
        .json()
        .unwrap();
    let key_1 = usergame_with_key_1.key.clone();
    println!("usergame_with_key_1: {:#?}", usergame_with_key_1);
    let usergame_with_key_2: GameWithKey = Client::new()
        .post(&api_url("/users/user_2/games/luck/key"))
        .header("Authorization", user_2.jwt.clone())
        .send()
        .unwrap()
        .json()
        .unwrap();
    let key_2 = usergame_with_key_2.key.clone();
    println!("usergame_with_key_2: {:#?}", usergame_with_key_2);
    let usergame_with_key_3: GameWithKey = Client::new()
        .post(&api_url("/users/user_3/games/luck/key"))
        .header("Authorization", user_3.jwt.clone())
        .send()
        .unwrap()
        .json()
        .unwrap();
    let key_3 = usergame_with_key_3.key.clone();
    println!("usergame_with_key_3: {:#?}", usergame_with_key_3);
    // Launch game
    struct WsClient {
        sender: Sender,
        username: String,
        user_game_key: String,
    }
    impl Handler for WsClient {
        fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
            let req = format!(
                "{}{}{}{}{}",
                r#"{"code":"findplay", "data": { "findplay" : {"username":""#,
                self.username,
                r#"", "user_game_key":""#,
                self.user_game_key,
                r#""}}}"#
            );
            self.sender.send(req)
        }
        fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
            println!("Got message: {}", msg);
            Ok(())
        }
    }
    let websocket_url_1 = game.websocket_url.clone();
    let websocket_url_2 = game.websocket_url.clone();
    let websocket_url_3 = game.websocket_url.clone();
    let thread_1 = thread::spawn(move || {
        ws::connect(websocket_url_1, |sender| WsClient {
            sender: sender,
            username: "user_1".to_owned(),
            user_game_key: key_1.to_owned(),
        })
        .unwrap();
    });
    let thread_2 = thread::spawn(move || {
        ws::connect(websocket_url_2, |sender| WsClient {
            sender: sender,
            username: "user_2".to_owned(),
            user_game_key: key_2.to_owned(),
        })
        .unwrap();
    });
    let thread_3 = thread::spawn(move || {
        ws::connect(websocket_url_3, |sender| WsClient {
            sender: sender,
            username: "user_3".to_owned(),
            user_game_key: key_3.to_owned(),
        })
        .unwrap();
    });
    let result_1 = thread_1.join();
    let result_2 = thread_2.join();
    let result_3 = thread_3.join();
}
