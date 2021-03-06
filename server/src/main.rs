#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

extern crate bcrypt;
extern crate glicko2;
extern crate jsonwebtoken as jwt;

use bcrypt::{hash, verify, DEFAULT_COST};
use jwt::{decode, encode, Header, Validation};
use rocket::http::Method;
use rocket::http::Status;
use rocket::Outcome;
use rocket::{get, routes};
use rocket_contrib::databases::postgres;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
//use rocket/*::{self, Request, FromRequest}*/;
use glicko2::{GameResult, Glicko2Rating};
use rand::{self, Rng};
use std::collections::HashMap;

mod data;
mod rating;
mod response;
mod types;
use data::Play;
use response::{Response, ResponseData};
mod errors;
use errors::*;
use rating::*;
use types::*;

static KEY: &str = "secret";
static GLICKO_SYS_CONSTANT: f64 = 0.5;
static MARGIN_FOR_WIN: f64 = 0.01;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct GameDesigner {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct NewUser<'a> {
    username: &'a str,
    password: &'a str,
    email: &'a str,
}

#[derive(Serialize)]
struct UserWithJwt {
    username: String,
    email: String,
    jwt: String,
}

#[derive(Serialize, Deserialize)]
struct GaimeError {
    status: String,
    error: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct FindPlay {
    username: Username,
    game_string_id: GameStringId,
}

#[derive(Serialize, Deserialize)]
struct NewPassword {
    password_1: String,
    password_2: String,
    new_password: String,
}

#[derive(Serialize, Deserialize)]
struct NewEmail {
    new_email: String,
}

#[derive(Serialize, Deserialize)]
struct Game {
    string_id: GameStringId,
    name: String,
    description: String,
    websocket_url: String,
    player_count: i32,
}

#[derive(Serialize, Deserialize)]
struct GameWithKey {
    key: String,
    string_id: GameStringId,
}

#[database("gaime")]
struct DatabaseConnection(postgres::Connection);

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Debug)]
enum JwtError {
    NoJwt,
}

fn jwt_from_username(username: &str) -> Result<String> {
    let exp = 10000000000;
    let claims = Claims {
        username: username.to_owned(),
        exp: exp,
    };
    let jwt = encode(&Header::default(), &claims, KEY.as_ref())?;
    Ok(jwt)
}

fn user_from_jwt_token(
    database_connection: &DatabaseConnection,
    jwt_token: &str,
) -> Result<Option<User>> {
    let token_data = decode::<Claims>(&jwt_token, b"secret", &Validation::default())?;
    let token_username = token_data.claims.username;
    let results = database_connection.query(
        "
        Select username, email
        From users
        Where username=$1;",
        &[&token_username],
    )?;
    match results.len() {
        1 => Ok(Some(User {
            username: results.get(0).get(0),
            email: results.get(0).get(1),
        })),
        _ => Ok(None),
    }
    /*match decode::<Claims>(&jwt_token, b"secret", &Validation::default()) {
        Ok(token_data) => {

        },
        Err(error) => {
            Err(error)
        }
    }*/
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for User {
    type Error = JwtError;
    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match request.guard::<DatabaseConnection>() {
            Outcome::Success(database_connection) => {
                match request.headers().get_one("Authorization") {
                    Some(authorization_header) => {
                        let jwt_token = authorization_header.replace("Bearer ", "");
                        match user_from_jwt_token(&database_connection, &jwt_token) {
                            Ok(user_option) => match user_option {
                                Some(user) => Outcome::Success(user),
                                None => Outcome::Failure((Status::BadRequest, JwtError::NoJwt)),
                            },
                            Err(_) => Outcome::Failure((Status::BadRequest, JwtError::NoJwt)),
                        }
                    }
                    None => {
                        // TODO : better error
                        Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
                    }
                }
            }
            _ => {
                // TODO : better error
                Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
            }
        }
    }
}

fn is_game_designer(database_connection: &DatabaseConnection, user: &User) -> Result<bool> {
    let results = database_connection.query(
        "
        select is_game_designer
        from users
        where users.username = $1;",
        &[&user.username],
    )?;
    match results.len() {
        0 => {
            bail!(ErrorKind::UserNotFound);
        }
        1 => {
            let is_game_designer: bool = results.get(0).get(0);
            if is_game_designer {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => {
            bail!(ErrorKind::TooManyUsers);
        }
    }
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for GameDesigner {
    type Error = JwtError;
    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        match request.guard::<DatabaseConnection>() {
            Outcome::Success(database_connection) => {
                match request.headers().get_one("Authorization") {
                    Some(authorization_header) => {
                        let jwt_token = authorization_header.replace("Bearer ", "");
                        match user_from_jwt_token(&database_connection, &jwt_token) {
                            Ok(user_option) => {
                                match user_option {
                                    Some(user) => {
                                        match is_game_designer(&database_connection, &user) {
                                            Ok(is_game_designer) => {
                                                if is_game_designer {
                                                    Outcome::Success(GameDesigner {
                                                        username: user.username,
                                                        email: user.email,
                                                    })
                                                } else {
                                                    // TODO : better error
                                                    Outcome::Failure((
                                                        Status::BadRequest,
                                                        JwtError::NoJwt,
                                                    ))
                                                }
                                            }
                                            Err(error) => {
                                                // TODO : better error
                                                Outcome::Failure((
                                                    Status::BadRequest,
                                                    JwtError::NoJwt,
                                                ))
                                            }
                                        }
                                    }
                                    None => Outcome::Failure((Status::BadRequest, JwtError::NoJwt)),
                                }
                            }
                            Err(error) => {
                                // TODO : better error
                                Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
                            }
                        }
                    }
                    None => {
                        // TODO : better error
                        Outcome::Failure((Status::BadRequest, JwtError::NoJwt))
                    }
                }
            }
            _ => {
                // TODO : better error
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
fn games(database_connection: DatabaseConnection) -> Result<Json<Vec<Game>>> {
    let results = database_connection.query(
        "
        SELECT string_id, name, description, websocket_url, player_count
        FROM games;",
        &[],
    )?;
    let mut games = Vec::new();
    for row in results.iter() {
        let player_count: i32 = row.get(4);
        games.push(Game {
            string_id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            websocket_url: row.get(3),
            player_count: player_count,
        });
    }
    Ok(Json(games))
}

#[post("/games", data = "<game>")]
fn games_post(
    database_connection: DatabaseConnection,
    game_designer: GameDesigner,
    game: Json<Game>,
) -> Result<Json<Game>> {
    let game_designer_user_id: i32 = database_connection
        .query(
            "select user_id
        from users
        where users.username = $1",
            &[&game_designer.username],
        )?
        .get(0)
        .get(0);
    database_connection.execute(
        "INSERT INTO games (string_id, name, description, websocket_url, user_id, key_hash, player_count)
        VALUES ($1, $2, $3, $4, $5, '', $6);",
        &[
            &game.string_id,
            &game.name,
            &game.description,
            &game.websocket_url,
            &game_designer_user_id,
            &game.player_count
        ]
    )?;
    Ok(game)
}

#[post("/users/<username>/games/<game_string_id>/key")]
fn user_game_key_post(
    username: Username,
    game_string_id: GameStringId,
    database_connection: DatabaseConnection,
    user: User,
) -> Result<Json<GameWithKey>> {
    if username == user.username {
        Ok(())
    } else {
        Err(ErrorKind::UsernamesDoNotMatch)
    }?;
    let game_string_id_from_db: String = database_connection
        .query(
            "select string_id
        from games
        where games.string_id = $1",
            &[&game_string_id],
        )?
        .get(0)
        .get(0);
    let rand_number: i64 = rand::thread_rng().gen();
    let new_key = hash(&rand_number.to_string(), DEFAULT_COST)?;
    let new_key_hash = hash(&new_key, DEFAULT_COST)?;
    database_connection.execute(
        "insert into users_games_keys (user_id, game_id, key_hash)
            (select users.user_id, games.game_id, $3
            from users, games
            where users.username = $1
                and games.string_id = $2)
        on conflict (user_id, game_id)
            do update set key_hash = $3;",
        &[&user.username, &game_string_id_from_db, &new_key_hash],
    )?;
    Ok(Json(GameWithKey {
        key: new_key,
        string_id: game_string_id_from_db,
    }))
}

#[post("/games/<game_string_id>/newkey")]
fn games_newkey(
    game_string_id: GameStringId,
    database_connection: DatabaseConnection,
    game_designer: GameDesigner,
) -> Result<Json<GameWithKey>> {
    let game_string_id_from_db: String = database_connection
        .query(
            "select string_id
        from games
        where games.string_id = $1",
            &[&game_string_id],
        )?
        .get(0)
        .get(0);
    let rand_number: i64 = rand::thread_rng().gen();
    let new_key = hash(&rand_number.to_string(), DEFAULT_COST)?;
    let new_key_hash = hash(&new_key, DEFAULT_COST)?;
    database_connection.execute(
        "update games
        set key_hash=$1
        where string_id=$2;",
        &[&new_key_hash, &game_string_id_from_db],
    )?;
    Ok(Json(GameWithKey {
        key: new_key,
        string_id: game_string_id_from_db,
    }))
}

#[get("/games/<game_string_id>")]
fn game(
    game_string_id: GameStringId,
    database_connection: DatabaseConnection,
) -> Result<Json<Game>> {
    let results = database_connection.query(
        "SELECT name, description, websocket_url, player_count FROM games WHERE string_id=$1;",
        &[&game_string_id],
    )?;
    match results.len() {
        0 => {
            bail!(ErrorKind::GameNotFound(game_string_id));
        }
        1 => {
            let game_name: String = results.get(0).get(0);
            let game_description: String = results.get(0).get(1);
            let game_websocket_url: String = results.get(0).get(2);
            let player_count: i32 = results.get(0).get(3);
            Ok(Json(Game {
                string_id: game_string_id,
                name: game_name,
                description: game_description,
                websocket_url: game_websocket_url,
                player_count: player_count,
            }))
        }
        _ => {
            bail!(ErrorKind::TooManyGames("to many games".to_owned()));
        }
    }
}

#[post("/login", data = "<login>")]
fn login(database_connection: DatabaseConnection, login: Json<Login>) -> Result<Json<UserWithJwt>> {
    let results = database_connection.query(
        "Select username, email, password_hash
        From users
        Where username=$1;",
        &[&login.username],
    )?;
    match results.len() {
        0 => {
            bail!(ErrorKind::UserNotFound);
        }
        1 => {
            let user_email: String = results.get(0).get(1);
            let user_password_hash: String = results.get(0).get(2);
            let user_password_hash_is_valid = verify(&login.password, &user_password_hash)?;
            match user_password_hash_is_valid {
                true => {
                    let jwt = jwt_from_username(&login.username)?;
                    Ok(Json(UserWithJwt {
                        username: login.username.clone(),
                        email: user_email,
                        jwt: jwt,
                    }))
                }
                false => {
                    bail!(ErrorKind::InvalidUsernamePasswordPair);
                }
            }
        }
        _ => {
            bail!(ErrorKind::TooManyUsers);
        }
    }
}

#[post("/users", data = "<new_user>")]
fn users_post(
    database_connection: DatabaseConnection,
    new_user: Json<NewUser>,
) -> Result<Json<UserWithJwt>> {
    let new_user_password_hash = hash(&new_user.password, DEFAULT_COST)?;
    database_connection.execute(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3);",
        &[&new_user.username, &new_user.email, &new_user_password_hash],
    )?;
    let jwt = jwt_from_username(new_user.username)?;
    Ok(Json(UserWithJwt {
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        jwt: jwt,
    }))
}

#[post("/email", data = "<new_email>")]
fn email_post(
    database_connection: DatabaseConnection,
    new_email: Json<NewEmail>,
    user: User,
) -> Result<Json<User>> {
    database_connection.execute(
        "UPDATE users
        SET email=$1
        WHERE username=$2;",
        &[&new_email.new_email, &user.username],
    )?;
    Ok(Json(User {
        username: user.username,
        email: new_email.new_email.to_owned(),
    }))
}

#[post("/password", data = "<new_password>")]
fn password_post(
    database_connection: DatabaseConnection,
    new_password: Json<NewPassword>,
    user: User,
) -> Result<Json<UserWithJwt>> {
    if new_password.password_1 == new_password.password_2 {
        Ok(())
    } else {
        Err(ErrorKind::PasswordsDoNotMatch)
    }?;
    let results = database_connection.query(
        "select username, email, password_hash
        from users
        where username=$1;",
        &[&user.username],
    )?;
    match results.len() {
        0 => {
            bail!(ErrorKind::UserNotFound);
        }
        1 => {
            let user_email: String = results.get(0).get(1);
            let user_password_hash: String = results.get(0).get(2);
            let password_is_valid = verify(&new_password.password_1, &user_password_hash)?;
            match password_is_valid {
                true => {
                    let password_hash = hash(&new_password.new_password, DEFAULT_COST)?;
                    database_connection.execute(
                        "update users
                        set password_hash=$1
                        where username=$2;",
                        &[&password_hash, &user.username],
                    )?;
                    let jwt = jwt_from_username(&user.username)?;
                    Ok(Json(UserWithJwt {
                        username: user.username,
                        email: user_email,
                        jwt: jwt,
                    }))
                }
                false => {
                    bail!(ErrorKind::InvalidUsernamePasswordPair);
                }
            }
        }
        _ => {
            bail!(ErrorKind::TooManyUsers);
        }
    }
}

#[post("/findplay", data = "<find_play_data>")]
fn findplay(
    database_connection: DatabaseConnection,
    find_play_data: Json<data::FindPlay>,
) -> Result<Json<Response>> {
    // Verify the game key
    let results = database_connection.query(
        "select games.game_id, games.key_hash, games.player_count
        from games
        where games.string_id = $1;",
        &[&find_play_data.game_string_id],
    )?;
    if results.len() > 0 {
        Ok(())
    } else {
        Err("Not enough results")
    }?;
    let game_id: GameId = results.get(0).get(0);
    let game_key_hash: String = results.get(0).get(1);
    let game_player_count: i32 = results.get(0).get(2);
    if verify(&find_play_data.game_key, &game_key_hash)? {
        Ok(())
    } else {
        Err(ErrorKind::InvalidGameKey)
    }?;
    // Verify the user game key
    let results = database_connection.query(
        "select users_games_keys.key_hash
        from users_games_keys, users, games
        where users.username = $1
            and games.string_id = $2
            and users_games_keys.game_id = games.game_id
            and users_games_keys.user_id = users.user_id;",
        &[&find_play_data.username, &find_play_data.game_string_id],
    )?;
    if results.len() > 0 {
        Ok(())
    } else {
        Err("Not enough results 2")
    }?;
    let user_game_key_hash: String = results.get(0).get(0);
    if verify(&find_play_data.user_game_key, &user_game_key_hash)? {
        // Do nothing
    } else {
        let code = response::Code::InvalidUserGameKey;
        let data = Some(ResponseData::findplay(find_play_data.into_inner()));
        return Ok(Json(Response::new(code, data)));
    }
    // Insert user into waiting pool
    database_connection.execute(
        "insert into users_waiting_for_games (game_id, user_id)
        select games.game_id, users.user_id
        from games, users
        where games.string_id = $1
            and users.username = $2;",
        &[&find_play_data.game_string_id, &find_play_data.username],
    )?;
    // Find other players waiting for the same game
    let users_results = database_connection.query(
        "select users.user_id, users.username
        from users_waiting_for_games, games, users
        where games.string_id = $1
            and users_waiting_for_games.game_id = games.game_id
            and users_waiting_for_games.user_id = users.user_id;",
        &[&find_play_data.game_string_id],
    )?;
    if users_results.len() >= game_player_count as usize {
        // Create new play
        let rand_number: i64 = rand::thread_rng().gen();
        let play_key = hash(&rand_number.to_string(), DEFAULT_COST)?;
        let play_key_hash = hash(&play_key, DEFAULT_COST)?;
        let results_2 = database_connection.query(
            "insert into plays (game_id, key_hash)
            values ($1, $2)
            returning play_id;",
            &[&game_id, &play_key_hash],
        )?;
        if results.len() > 0 {
            Ok(())
        } else {
            Err("Not enough results 3")
        }?;
        let play_id: PlayId = results_2.get(0).get(0);
        // Add users to play, and remove them from waiting pool
        let mut usernames = Vec::new();
        for i in 0..(game_player_count as usize) {
            let user_result_user_id: i32 = users_results.get(i).get(0);
            let user_result_username: String = users_results.get(i).get(1);
            database_connection.execute(
                "insert into users_in_plays (user_id, play_id)
                values ($1, $2);",
                &[&user_result_user_id, &play_id],
            )?;
            database_connection.execute(
                "delete from users_waiting_for_games
                where users_waiting_for_games.game_id = $1
                    and users_waiting_for_games.user_id = $2;",
                &[&game_id, &user_result_user_id],
            )?;
            usernames.push(user_result_username);
        }
        Ok(Json(Response::new(
            response::Code::PlayFound,
            Some(ResponseData::play(Play {
                play_id: play_id,
                usernames: usernames,
                key: play_key,
            })),
        )))
    } else {
        Ok(Json(Response::new(
            response::Code::WaitingForOpponent,
            None,
        )))
    }
}

#[post("/endplay", data = "<end_play_data>")]
fn endplay(
    database_connection: DatabaseConnection,
    end_play_data: Json<data::EndPlay>,
) -> Result<Json<Response>> {
    // Verify that we are authorized to end the play
    // The key needs to be validated against the hash stored in the database
    let query_results = database_connection.query(
        "select plays.key_hash
        from plays
        where plays.play_id = $1;",
        &[&end_play_data.play_id],
    )?;
    let play_key_hash: String = query_results.get(0).get(0);
    if verify(&end_play_data.play_key, &play_key_hash)? {
        Ok(())
    } else {
        Err(ErrorKind::InvalidPlayKey)
    }?;
    // Verify that each player knows it's game key
    for (username, user_data) in end_play_data.users_data.iter() {
        let query_results = database_connection.query(
            "select users_games_keys.key_hash
            from users_games_keys, users, plays
            where users.username = $1
                and plays.play_id = $2
                and users_games_keys.user_id = users.user_id
                and users_games_keys.game_id = plays.game_id;",
            &[&user_data.username, &end_play_data.play_id],
        )?;
        if query_results.len() > 0 {
            Ok(())
        } else {
            Err(ErrorKind::InvalidUserGameKey)
        }?;
        let user_game_key_hash: Key = query_results.get(0).get(0);
        if verify(&user_data.user_game_key, &user_game_key_hash)? {
            Ok(())
        } else {
            Err(ErrorKind::InvalidUserGameKey)
        }?;
    }
    // Get game_id
    let game_id: GameId = database_connection
        .query(
            "select game_id
        from plays
        where plays.play_id = $1;",
            &[&end_play_data.play_id],
        )?
        .get(0)
        .get(0);
    // Init ratings if they don't exist
    let initial_rating = Glicko2Rating::unrated();
    for username in end_play_data.users_data.keys() {
        database_connection.execute(
            "insert into users_ratings (game_id, user_id, glicko2_value, glicko2_deviation, glicko2_volatility)
            select $1, users.user_id, $2, $3, $4
            from users
            where users.username = $5
            on conflict
                do nothing;",
            &[
                &game_id,
                &initial_rating.value,
                &initial_rating.deviation,
                &initial_rating.volatility,
                &username]
        )?;
    }
    // Retrieve data for users in this play
    let users_data = database_connection.query(
        "select users_ratings.game_id, users.user_id, users.username,
            users_ratings.glicko2_value, users_ratings.glicko2_deviation, users_ratings.glicko2_volatility
        from users, users_in_plays, users_ratings, plays
        where users_in_plays.play_id = $1
            and plays.play_id = $1
            and users.user_id = users_in_plays.user_id
            and users_ratings.game_id = plays.game_id
            and users_ratings.user_id = users.user_id;",
        &[&end_play_data.play_id]
    )?;
    // Setup ratings before calculation
    let mut old_glicko2_ratings = HashMap::new();
    let mut new_glicko2_ratings = HashMap::new();
    for user_data in users_data.iter() {
        let username: String = user_data.get(2);
        let glicko2_value: f64 = user_data.get(3);
        let glicko2_deviation: f64 = user_data.get(4);
        let glicko2_volatility: f64 = user_data.get(5);
        old_glicko2_ratings.insert(
            username.clone(),
            Glicko2Rating {
                value: glicko2_value,
                deviation: glicko2_deviation,
                volatility: glicko2_volatility,
            },
        );
        new_glicko2_ratings.insert(
            username.clone(),
            Glicko2Rating {
                value: glicko2_value,
                deviation: glicko2_deviation,
                volatility: glicko2_volatility,
            },
        );
    }
    // Calculate ratings
    for (username_1, user_data_1) in end_play_data.users_data.iter() {
        let mut results = Vec::new();
        let prior_glicko2_rating = *old_glicko2_ratings.get(username_1).unwrap();
        for (username_2, user_data_2) in end_play_data.users_data.iter() {
            if username_1 != username_2 {
                let glicko2_rating_2 = *old_glicko2_ratings.get(username_2).unwrap();
                let rank_diff = user_data_1.rank - user_data_2.rank;

                let game_result = if rank_diff < -MARGIN_FOR_WIN {
                    // username_1 win
                    GameResult::win(glicko2_rating_2)
                } else if rank_diff > MARGIN_FOR_WIN {
                    // username_1 lose
                    GameResult::loss(glicko2_rating_2)
                } else {
                    // draw
                    GameResult::draw(glicko2_rating_2)
                };
                results.push(game_result);
            } else {
                // Do nothing
            }
        }
        new_glicko2_ratings.insert(
            username_1.to_string(),
            glicko2::new_rating(prior_glicko2_rating, &results, GLICKO_SYS_CONSTANT),
        );
    }
    // Convert Glicko2Rating to MyGlicko2Rating for serialisation
    let mut ratings = HashMap::new();
    for username in end_play_data.users_data.keys() {
        let old_rating = old_glicko2_ratings.get(username).unwrap();
        let new_rating = new_glicko2_ratings.get(username).unwrap();
        ratings.insert(
            username.to_string(),
            Rating {
                old_rating: MyGlicko2Rating {
                    value: old_rating.value,
                    deviation: old_rating.deviation,
                    volatility: old_rating.volatility,
                },
                new_rating: MyGlicko2Rating {
                    value: new_rating.value,
                    deviation: new_rating.deviation,
                    volatility: new_rating.volatility,
                },
            },
        );
    }
    // Update user_ratings
    for user_data in users_data.iter() {
        let user_id: i32 = user_data.get(1);
        let username: String = user_data.get(2);
        let new_rating = new_glicko2_ratings.get(&username).unwrap();
        let glicko2_value: f64 = new_rating.value;
        let glicko2_deviation: f64 = new_rating.deviation;
        let glicko2_volatility: f64 = new_rating.volatility;
        database_connection.execute(
            "update users_ratings
            set glicko2_value = $1, glicko2_deviation = $2, glicko2_volatility = $3 
            where user_id = $4
                and game_id = $5",
            &[
                &glicko2_value,
                &glicko2_deviation,
                &glicko2_volatility,
                &user_id,
                &game_id,
            ],
        )?;
    }
    // Insert play results in plays_results
    for user_data in users_data.iter() {
        let user_id: i32 = user_data.get(1);
        let username: String = user_data.get(2);
        let user_rank: Rank = end_play_data.users_data.get(&username).unwrap().rank;
        database_connection.execute(
            "insert into plays_results (play_id, user_id, user_rank)
            values ($1, $2, $3)",
            &[&end_play_data.play_id, &user_id, &user_rank],
        )?;
    }
    // Delete users from users_in_plays
    database_connection.execute(
        "delete from users_in_plays
        where users_in_plays.play_id = $1;",
        &[&end_play_data.play_id],
    )?;
    // Send response with old and new ratings for each user
    Ok(Json(Response::new(
        response::Code::EndPlayOk,
        Some(ResponseData::endplay(data::EndPlayOk {
            game_id: game_id,
            play_id: end_play_data.play_id,
            ratings: ratings,
        })),
    )))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() -> Result<()> {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:4200"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;
    rocket::ignite()
        .mount(
            "/",
            routes![
                email_post,
                password_post,
                endplay,
                findplay,
                game,
                games,
                games_post,
                games_newkey,
                login,
                users_post,
                userself,
                user_game_key_post
            ],
        )
        .attach(cors)
        .attach(DatabaseConnection::fairing())
        .launch();
    Ok(())
}
