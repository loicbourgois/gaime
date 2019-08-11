#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_contrib::json::{Json, JsonValue};


type GameId = String;


#[derive(Serialize, Deserialize)]
struct Game {
    game_id: GameId,
    name: String
}


#[get("/games")]
fn games() -> JsonValue {
    json!({
        "games": [
                Game {
                    game_id: "snake".to_string(),
                    name: "Snake".to_string()
                }
        ]
    })
}


#[get("/game/<game_id>")]
fn game(game_id: GameId) -> Option<Json<Game>> {
    match game_id.as_ref() {
        "snake" => {
            Some(
                Json(
                    Game {
                        game_id: game_id,
                        name: "Snake".to_string()
                    }
                )
            )
        },
        _ => {
            None
        }
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

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![games, game])
        .attach(cors)
        .launch();

    Ok(())
}

