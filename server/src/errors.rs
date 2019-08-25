//#[macro_use] extern crate error_chain;

error_chain! {
    foreign_links {
        RocketPostgresError(rocket_contrib::databases::postgres::Error);
        Jwt(jwt::errors::Error);
        //NoJwt2(JwtError::NoJwt);
        Fmt(::std::fmt::Error);
        BcryptError(bcrypt::BcryptError);
        RocketCorsError(rocket_cors::Error);
    }
    errors {
        GameNotFound(s: String) {
            description("game not found")
            display("game not found: '{}'", s)
        }
        TooManyGames(s: String) {
            description("too many games")
            display("too many games: '{}'", s)
        }
        UserNotFound {
            description("user not found")
        }
        InvalidUsernamePasswordPair {
            description("invalid username/password pair")
        }
        TooManyUsers {
            description("too many users")
        }
        PasswordsDoNotMatch {
            description("passwords do not match")
        }
        UsernamesDoNotMatch {
            description("usernames do not match")
        }
        InvalidGameKey {
            description("invalid game key")
        }
        InvalidPlayKey {
            description("invalid play key")
        }
        InvalidUserGameKey {
            description("invalid user game key")
        }
    }
}
