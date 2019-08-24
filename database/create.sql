CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR(128) UNIQUE NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    password_hash VARCHAR(128) NOT NULL,
    is_game_designer BOOLEAN NOT NULL DEFAULT FALSE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS games (
    game_id SERIAL PRIMARY KEY NOT NULL,
    user_id integer REFERENCES users(user_id) NOT NULL,
    string_id VARCHAR(128) UNIQUE NOT NULL,
    name VARCHAR(128) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    websocket_url TEXT NOT NULL,
    key_hash VARCHAR(128) NOT NULL,
    player_count integer NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS plays (
    play_id SERIAL PRIMARY KEY,
    game_id integer REFERENCES games (game_id) NOT NULL,
    key_hash VARCHAR(128) NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS plays_results (
    play_id integer REFERENCES users(user_id) NOT NULL,
    user_id integer REFERENCES users(user_id) NOT NULL,
    user_rank float NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (play_id, user_id)
);

CREATE TABLE IF NOT EXISTS users_in_plays (
    user_id integer REFERENCES users(user_id) NOT NULL,
    play_id integer REFERENCES plays(play_id) NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, play_id)
);

CREATE TABLE IF NOT EXISTS users_waiting_for_games (
    game_id integer REFERENCES games(game_id) NOT NULL,
    user_id integer REFERENCES users(user_id) NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, game_id)
);

CREATE TABLE IF NOT EXISTS users_ratings (
    user_id integer REFERENCES users(user_id) NOT NULL,
    game_id integer REFERENCES games(game_id) NOT NULL,
    glicko2_value float NOT NULL,
    glicko2_deviation float NOT NULL,
    glicko2_volatility float NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, game_id)
);

CREATE TABLE IF NOT EXISTS users_games_keys (
    user_id integer REFERENCES users(user_id) NOT NULL,
    game_id integer REFERENCES games(game_id) NOT NULL,
    key_hash VARCHAR(128) NOT NULL,
    created_date TIMESTAMP NOT NULL DEFAULT now(),
    modified_date TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, game_id)
);

