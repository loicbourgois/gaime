extern crate ws;
extern crate rand;

#[macro_use] extern crate serde_derive;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};
use serde_json::{Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};
use std::cell::RefMut;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::fmt;
use rand::Rng;

#[derive(Serialize, Deserialize)]
struct FindGameData {
    user: User,
    game_string_id: String
}

#[derive(Serialize, Deserialize)]
struct GameCommandData {
    user: User,
    game_command: GameCommand
}

type Username = String;

#[derive(Serialize, Deserialize)]
enum GameCommand {
    Up,
    Down,
    Left,
    Right
}

impl GameCommand {
    fn to_string(&self) -> &str {
        match self {
            GameCommand::Up => {
                "Up"
            },
            GameCommand::Down => {
                "Down"
            },
            GameCommand::Left => {
                "Left"
            },
            GameCommand::Right => {
                "Right"
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: Username,
    jwt: String
}

struct WebSocketHandler {
    sender: Sender,
    user: Option<User>,
    waiting_users: Rc<RefCell<HashMap<Username, User>>>,
    playing_users: Arc<Mutex<HashMap<Username, User>>>,
    senders: Rc<RefCell<HashMap<Username, Sender>>>,
    game_commands: Arc<Mutex<HashMap<Username, GameCommand>>>
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
                let playing_users_locked = self.playing_users.lock().unwrap();
                match playing_users_locked.get(&find_game_data.user.username) {
                    Some(user) => {
                        self.send_error(&format!("User {} already in game", user.username))
                    },
                    None => {
                        drop(playing_users_locked);
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
                                self.find_opponent(&mut waiting_users)
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

    fn pass_game_command(&mut self, data: String) -> Result<()> {
        match serde_json::from_str::<GameCommandData>(&data) {
            Ok(game_command_data) => {
                let mut game_commands_locked = self.game_commands.lock().unwrap();
                let username = game_command_data.user.username.clone();
                game_commands_locked.insert(username, game_command_data.game_command);
                drop(game_commands_locked);
                self.sender.send("ok")
            },
            Err(error) => {
                println!("Error: {}", data);
                self.send_error(&error.to_string())
            }
        }
    }

    fn find_opponent(
        & self,
        waiting_users: &mut RefMut<HashMap<Username, User>>
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
                                        self.playing_users.lock().unwrap().insert(player_1_username.to_string(), player_1.clone());
                                        self.playing_users.lock().unwrap().insert(player_2_username.to_string(), player_2.clone());
                                        match self.senders.borrow().clone().get(player_2_username) {
                                            Some(player_2_sender) => {
                                                let player_1_sender_game = self.sender.clone();
                                                let player_2_sender_game = player_2_sender.clone();
                                                let playing_users = Arc::clone(&self.playing_users);
                                                let game_commands = Arc::clone(&self.game_commands);
                                                let handle = thread::spawn(|| {
                                                    Game::new(
                                                        player_1,
                                                        player_2,
                                                        player_1_sender_game,
                                                        player_2_sender_game,
                                                        playing_users,
                                                        game_commands
                                                    ).run();
                                                });
                                                //
                                                // TODO
                                                // spin new thread with
                                                // - player_1_sender
                                                // - player_2_sender
                                                // - player_1
                                                // - player_2
                                                //
                                                //handle.join().unwrap();
                                                match player_2_sender.send("Opponent found") {
                                                    Ok(_ok) => {
                                                    },
                                                    Err(error) => {
                                                        println!("{}", error);
                                                    }
                                                };
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
        let message = msg.as_text().unwrap();
        let message_json: Value = serde_json::from_str(message).unwrap();
        match message_json["command"].as_str() {
            Some(command) => {
                match command {
                    "findgame" => {
                        self.find_game(message_json["data"].to_string())
                    },
                    "gamecommand" => {
                        self.pass_game_command(message_json["data"].to_string())
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
            _ => println!("The client encountered an error: {} | {:?}", reason, code),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Game {
    player_1: User,
    player_2: User,
    player_1_sender: Sender,
    player_2_sender: Sender,
    playing_users: Arc<Mutex<HashMap<Username, User>>>,
    game_commands: Arc<Mutex<HashMap<Username, GameCommand>>>
}

type X = usize;
type Y = usize;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct BodyPart {
    x: X,
    y: Y
}

#[derive(Serialize, Deserialize, Clone)]
struct Snake {
    body_parts: Vec<BodyPart>,
    username: Username,
    direction: Direction,
    is_alive: bool
}

#[derive(Serialize, Deserialize, Clone)]
struct Food {
    x: usize,
    y: usize
}

#[derive(Serialize, Deserialize, Clone)]
struct GameState {
    width: usize,
    height: usize,
    foods: Vec<Food>,
    snakes: HashMap<Username, Snake>
}

#[derive(Serialize, Deserialize, Clone)]
struct GameStateResponse<'a> {
    status: &'a str,
    response_type: &'a str,
    game_state: GameState
}

impl GameStateResponse<'_> {
    fn new<'a>(game_state: GameState) -> GameStateResponse<'a> {
        GameStateResponse {
            status: "ok",
            response_type: "game_state",
            game_state: game_state
        }
    }

    fn as_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Game {

    fn new(
        player_1: User,
        player_2: User,
        player_1_sender: Sender,
        player_2_sender: Sender,
        playing_users: Arc<Mutex<HashMap<Username, User>>>,
        game_commands: Arc<Mutex<HashMap<Username, GameCommand>>>
    ) -> Game {
        Game {
            player_1,
            player_2,
            player_1_sender,
            player_2_sender,
            playing_users,
            game_commands: game_commands
        }
    }

    fn run(&self) {
        self.send_message_to_all("Game Start");
        let mut playing_users_locked = self.playing_users.lock().unwrap();
        playing_users_locked.remove(&self.player_1.username);
        playing_users_locked.remove(&self.player_2.username);
        drop(playing_users_locked);
        let mut elapsed_steps = 0;
        let mut go = true;
        let mut foods = Vec::new();
        let width = 21;
        let height = 21;
        let mut rng = rand::thread_rng();
        let max_food_count = 3;
        let mut food_count = 0;
        while food_count < max_food_count {
            foods.push(Food {
                x: rng.gen_range(0, width),
                y: rng.gen_range(0, height)
            });
            food_count += 1;
        }
        let mut body_parts_1 = Vec::new();
        body_parts_1.push(BodyPart{
            x: (width - 1) / 4,
            y: (height - 1) / 4
        });
        body_parts_1.push(BodyPart{
            x: (width - 1) / 4 - 1,
            y: (height - 1) / 4
        });
        let mut body_parts_2 = Vec::new();
        body_parts_2.push(BodyPart{
            x: width - 1 - (width - 1) / 4,
            y: height - 1 - (height - 1) / 4
        });
        body_parts_2.push(BodyPart{
            x: width - 1 - (width - 1) / 4 + 1,
            y: height - 1 - (height - 1) / 4
        });
        let mut snakes = HashMap::new();
        snakes.insert(
            self.player_1.username.clone(),
            Snake {
                username: self.player_1.username.clone(),
                body_parts: body_parts_1,
                direction: Direction::Right,
                is_alive: true
            }
        );
        snakes.insert(
            self.player_2.username.clone(),
            Snake {
                username: self.player_2.username.clone(),
                body_parts: body_parts_2,
                direction: Direction::Left,
                is_alive: true
            }
        );
        let mut game_state = GameState {
            height: height,
            width: width,
            foods: foods,
            snakes: snakes
        };
        self.send_message_to_all(&GameStateResponse::new(game_state.clone()).as_json_string());
        for (k, snake) in game_state.snakes.clone().iter() {
            let mut game_commands_locked = self.game_commands.lock().unwrap();
            game_commands_locked.remove(&snake.username);
        }
        while go {
            thread::sleep(Duration::from_millis(250));
            // Treat commands
            for (k, snake) in game_state.snakes.clone().iter() {
                let game_commands_locked = self.game_commands.lock().unwrap();
                match game_commands_locked.get(&snake.username) {
                    Some(command) => {
                        match command {
                            GameCommand::Up => {
                                game_state.snakes.get_mut(k).unwrap().direction = Direction::Up;
                            },
                            GameCommand::Down => {
                                game_state.snakes.get_mut(k).unwrap().direction = Direction::Down;
                            },
                            GameCommand::Left => {
                                game_state.snakes.get_mut(k).unwrap().direction = Direction::Left;
                            },
                            GameCommand::Right => {
                                game_state.snakes.get_mut(k).unwrap().direction = Direction::Right;
                            }
                        };
                    }, None => {
                        // Do nothing
                    }
                };
                drop(game_commands_locked);
            }
            // update snake positions
            for (k, snake) in game_state.snakes.clone().iter() {
                if snake.is_alive {
                    let c = snake.body_parts.len();
                    for i in (1..c).rev() {
                        game_state.snakes.get_mut(k).unwrap().body_parts[i].x = snake.body_parts[i-1].x;
                        game_state.snakes.get_mut(k).unwrap().body_parts[i].y = snake.body_parts[i-1].y;
                    }
                    match game_state.snakes.get(k).unwrap().direction {
                        Direction::Up => {
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].y += game_state.height - 1;
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].y %= game_state.height;
                        },
                        Direction::Down => {
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].y += game_state.height + 1;
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].y %= game_state.height;
                        },
                        Direction::Left => {
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].x += game_state.width - 1;
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].x %= game_state.width;
                        },
                        Direction::Right => {
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].x += game_state.width + 1;
                            game_state.snakes.get_mut(k).unwrap().body_parts[0].x %= game_state.width;
                        }
                    }
                } else {
                    // Do nothing
                }
            }
            // check snake - snake colision
            for (k1, snake1) in game_state.snakes.clone().iter() {
                for (k2, snake2) in game_state.snakes.clone().iter() {
                    //let l1 = snake1.body_parts.len();
                    let head_1_x = snake1.body_parts[0].x;
                    let head_1_y = snake1.body_parts[0].y;
                    let l2 = snake2.body_parts.len();
                    for i in 0..l2 {
                        if k1 == k2 && i == 0 {
                            // Do nothing
                        } else {
                            let snake2_x = snake2.body_parts[i].x;
                            let snake2_y = snake2.body_parts[i].y;
                            if head_1_x == snake2_x && head_1_y == snake2_y {
                                game_state.snakes.get_mut(k1).unwrap().is_alive = false;
                            } else {
                                // Do nothing
                            }
                        }
                    }
                }
            }
            // check snake - food colision
            let mut foods_to_remove = Vec::new();
            for (k, snake) in game_state.snakes.clone().iter() {
                if snake.is_alive {
                    for (food_index, food) in game_state.foods.clone().iter().enumerate() {
                        if snake.body_parts[0].x == food.x && snake.body_parts[0].y == food.y {
                            let l = snake.body_parts.len();
                            game_state.snakes.get_mut(k).unwrap().body_parts.push(BodyPart {
                                x: snake.body_parts[l-1].x,
                                y: snake.body_parts[l-1].y
                            });
                            foods_to_remove.push(food_index);
                        } else {
                            // Do nothing
                        }
                    }
                } else {
                    // Do nothing
                }
            }
            //
            for food_index in foods_to_remove.iter().rev() {
                game_state.foods.remove(*food_index);
            }
            //
            let mut food_count = game_state.foods.len();
            while food_count < max_food_count {
                game_state.foods.push(Food {
                    x: rng.gen_range(0, width),
                    y: rng.gen_range(0, height)
                });
                food_count += 1;
            }
            let mut alive_count = 0;
            for (k, snake) in game_state.snakes.clone().iter() {
                if snake.is_alive {
                    alive_count += 1;
                } else {
                    // Do nothing
                }
            }
            if alive_count < 2 {
                go = false;
            } else {
                // Do nothing;
            }
            elapsed_steps += 1;
            self.send_message_to_all(&GameStateResponse::new(game_state.clone()).as_json_string());
        }
        self.send_message_to_all("Game End");
    }

    fn send_message_to_all(&self, message: &str) {
        match self.player_1_sender.send(message) {
            Ok(_ok) => {
            },
            Err(error) => {
                println!("{}", error);
            }
        };
        match self.player_2_sender.send(message) {
            Ok(_ok) => {
            },
            Err(error) => {
                println!("{}", error);
            }
        };
    }
}

fn main() -> Result<()> {
    let waiting_users = Rc::new(RefCell::new(HashMap::new()));
    let playing_users = Arc::new(Mutex::new(HashMap::new()));
    let senders = Rc::new(RefCell::new(HashMap::new()));
    let game_commands = Arc::new(Mutex::new(HashMap::new()));
    let url = "0.0.0.0:8080";
    match listen(
        url,
        |sender| WebSocketHandler {
            sender: sender,
            user: None,
            waiting_users: waiting_users.clone(),
            playing_users: Arc::clone(&playing_users),
            senders: senders.clone(),
            game_commands: Arc::clone(&game_commands)
        }
    ) {
        Ok(ok) => {
            println!("Server listening on {}", url);
            Ok(ok)
        },
        Err(error) => {
            println!("Could not start server on {}", url);
            println!("Error: {}", error);
            Err(error)
        }
    }
}
