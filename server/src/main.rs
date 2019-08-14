#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate jsonwebtoken as jwt;
extern crate bcrypt;


use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::databases::postgres;
use jwt::{encode, decode, Header, Validation};
use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};


type GameStringId = String;


#[derive(Serialize, Deserialize)]
struct GaimeError {
    status: String,
    error: String,
    message: String
}


#[derive(Serialize, Deserialize)]
struct NewPassword {
    password_1: String,
    password_2: String,
    new_password: String
}


#[derive(Serialize, Deserialize)]
struct NewEmail {
    new_email: String
}


#[derive(Serialize, Deserialize)]
struct Game {
    string_id: GameStringId,
    name: String,
    description: String,
    websocket_url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize
}


#[database("gaime")]
struct DatabaseConnection(postgres::Connection);


#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String
}


#[derive(Serialize, Deserialize)]
struct Signup {
    username: String,
    password: String,
    email: String
}


#[derive(Serialize, Deserialize)]
struct Login {
    username: String,
    password: String
}



#[derive(Debug)]
enum JwtError {
    NoJwt
}


fn user_from_jwt_token(database_connection: &DatabaseConnection, jwt_token: &str) -> Result<Option<User>, postgres::Error> {
    let token_data = decode::<Claims>(&jwt_token, b"secret", &Validation::default());
    let token_username = token_data.unwrap().claims.username;
    match database_connection.query(
        "Select username, email From users Where username=$1;",
        &[&token_username]
    ) {
        Ok(results) => {
            match results.len() {
                1 => {
                    Ok(Some(User {
                        username: results.get(0).get(0),
                        email: results.get(0).get(1)
                    }))
                },
                _ => {
                    Ok(None)
                }
            }
        },
        Err(error) => {
            Err(error)
        }
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = JwtError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let database_connection = request.guard::<DatabaseConnection>().unwrap();
        let authorization_header = request.headers().get_one("Authorization").unwrap();
        let jwt_token = authorization_header.replace("Bearer ", "");
        match user_from_jwt_token(&database_connection, &jwt_token) {
            Ok(user_option) => {
                match user_option {
                    Some(user) => {
                        Outcome::Success(user)
                    },
                    None => {
                        Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
                    }
                }
            },
            Err(_) => {
                Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
            }
        }
    }
}


#[get("/self")]
fn userself(user: User) -> Json<User> {
    Json(user)
}


#[get("/games")]
fn games(database_connection: DatabaseConnection) -> Result<Json<Vec<Game>>, JsonValue> {
    
    match database_connection.query(
        "SELECT string_id, name, description, websocket_url FROM games;",
        &[]
    ) {
        Ok(results) => {
            let mut games = Vec::new();
            for row in results.iter() {
                games.push(Game {
                    string_id: row.get(0),
                    name: row.get(1),
                    description: row.get(2),
                    websocket_url: row.get(3)
                });
            }
            Ok(Json(games))
        },
        Err(error) => {
            Err(json!(GaimeError {
                status: "error".to_owned(),
                error: error.to_string(),
                message: error.to_string()
            }))
        }
    }
}


#[get("/game/<game_string_id>")]
fn game(game_string_id: GameStringId, database_connection: DatabaseConnection) -> Result<Json<Game>, JsonValue> {
    match database_connection.query(
        "SELECT name, description, websocket_url FROM games WHERE string_id=$1;",
        &[&game_string_id]
    ) {
        Ok(results) => {
            match results.len() {
                0 => {
                    Err(json!(GaimeError {
                        status: "error".to_owned(),
                        error: "game_does_not_exist".to_owned(),
                        message: format!("Game {} does not exist", game_string_id)
                    }))
                },
                1 => {
                    let game_name: String = results.get(0).get(0);
                    let game_description: String = results.get(0).get(1);
                    let game_websocket_url: String = results.get(0).get(2);
                    Ok(Json(Game {
                        string_id: game_string_id,
                        name: game_name,
                        description: game_description,
                        websocket_url: game_websocket_url
                    }))
                },
                _ => {
                    Err(json!(GaimeError {
                        status: "error".to_owned(),
                        error: "too_many_games".to_owned(),
                        message: format!("Found more than 1 game with string_id {}", game_string_id)
                    }))
                }
            }
        },
        Err(error) => {
            Err(json!(GaimeError {
                status: "error".to_owned(),
                error: error.to_string(),
                message: error.to_string()
            }))
        }
    }
}


#[post("/login", data = "<login>")]
fn login(database_connection: DatabaseConnection, login: Json<Login>) -> JsonValue {
    let key = "secret";
    let exp = 10000000000;
    let claims = Claims {
        username: login.username.to_owned(),
        exp: exp
    };
    match database_connection.query(
        "Select username, email, hash From users Where username=$1;",
        &[&login.username]
    ) {
        Ok(results) => {
            match results.len() {
                0 => {
                    json!({
                        "status": "error",
                        "error": format!("User {} does not exist", login.username)
                    })
                },
                1 => {
                    let user_hash: String = results.get(0).get(2);
                    match verify(&login.password, &user_hash) {
                        Ok(is_valid) => {
                            match is_valid {
                                true => {
                                    match encode(&Header::default(), &claims, key.as_ref()) {
                                        Ok(jwt_token) => {
                                            json!({
                                                "status": "ok",
                                                "jwt_token": jwt_token
                                            })
                                        },
                                        Err(error) => {
                                            json!({
                                                "status": "error",
                                                "error": error.to_string()
                                            })
                                        }
                                    }
                                },
                                false => {
                                    json!({
                                        "status": "error",
                                        "error": "Invalid username/password pair"
                                    })
                                }
                            }
                        },
                        Err(error) => {
                            json!({
                                "status": "error",
                                "error": error.to_string()
                            })
                        }
                    }
                },
                _ => {
                    json!({
                        "status": "error",
                        "error": format!("Found more than 1 user with username {}", login.username)
                    })
                }
            }
        },
        Err(error) => {
            json!({
                "status": "error",
                "error": error.to_string()
            })
        }
    }
}


#[post("/signup", data = "<signup>")]
fn signup(database_connection: DatabaseConnection, signup: Json<Signup>) -> JsonValue {
    match hash(&signup.password, DEFAULT_COST) {
        Ok(hash) => {
            match database_connection.execute(
                "INSERT INTO users (username, email, hash) VALUES ($1, $2, $3);",
                &[&signup.username, &signup.email, &hash]
            ) {
                Ok(_) => {
                    json!({ "status": "ok" })
                },
                Err(error) => {
                    json!({
                        "status": "error",
                        "error": error.to_string()
                    })
                }
            }
        },
        Err(error) => {
            json!({
                "status": "error",
                "error": error.to_string()
            })
        }
    }
}


#[post("/changeemail", data = "<new_email>")]
fn changeemail(database_connection: DatabaseConnection, new_email: Json<NewEmail>, user: User) -> JsonValue {
    match database_connection.execute(
        "UPDATE users SET email=$1 WHERE username=$2;",
        &[&new_email.new_email, &user.username]
    ) {
        Ok(_) => {
            json!({ "status": "ok" })
        },
        Err(error) => {
            json!({
                "status": "error",
                "error": error.to_string()
            })
        }
    }
}


#[post("/changepassword", data = "<new_password>")]
fn changepassword(database_connection: DatabaseConnection, new_password: Json<NewPassword>, user: User) -> JsonValue {
    let key = "secret";
    let exp = 10000000000;
    let claims = Claims {
        username: user.username.to_owned(),
        exp: exp
    };
    if new_password.password_1 == new_password.password_2 {
        match database_connection.query(
            "Select username, email, hash From users Where username=$1;",
            &[&user.username]
        ) {
            Ok(results) => {
                match results.len() {
                    0 => {
                        json!({
                            "status": "error",
                            "error": format!("User {} does not exist", user.username)
                        })
                    },
                    1 => {
                        let user_hash: String = results.get(0).get(2);
                        match verify(&new_password.password_1, &user_hash) {
                            Ok(is_valid) => {
                                match is_valid {
                                    true => {
                                        match hash(&new_password.new_password, DEFAULT_COST) {
                                            Ok(hash) => {
                                                match database_connection.execute(
                                                    "UPDATE users SET hash=$1 WHERE username=$2;",
                                                    &[&hash, &user.username]
                                                ) {
                                                    Ok(_) => {
                                                        match encode(&Header::default(), &claims, key.as_ref()) {
                                                            Ok(jwt_token) => {
                                                                json!({
                                                                    "status": "ok",
                                                                    "jwt_token": jwt_token
                                                                })
                                                            },
                                                            Err(error) => {
                                                                json!({
                                                                    "status": "error",
                                                                    "error": error.to_string()
                                                                })
                                                            }
                                                        }
                                                    },
                                                    Err(error) => {
                                                        json!({
                                                            "status": "error",
                                                            "error": error.to_string()
                                                        })
                                                    }
                                                }
                                            },
                                            Err(error) => {
                                                json!({
                                                    "status": "error",
                                                    "error": error.to_string()
                                                })
                                            }
                                        }
                                    },
                                    false => {
                                        json!({
                                            "status": "error",
                                            "error": "Invalid username/password pair"
                                        })
                                    }
                                }
                            },
                            Err(error) => {
                                json!({
                                    "status": "error",
                                    "error": error.to_string()
                                })
                            }
                        }
                    },
                    _ => {
                        json!({
                            "status": "error",
                            "error": format!("Found more than 1 user with username {}", user.username)
                        })
                    }
                }
            },
            Err(error) => {
                json!({
                    "status": "error",
                    "error": error.to_string()
                })
            }
        }
    } else {
            json!({
                "status": "error",
                "error": "Password 1 and Password 2 do not match"
            })
    }
}


#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


fn main() -> Result<(), Error> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![
            games,
            game,
            signup,
            login,
            userself,
            changeemail,
            changepassword
        ])
        .attach(cors)
        .attach(DatabaseConnection::fairing())
        .launch();

    Ok(())
}

