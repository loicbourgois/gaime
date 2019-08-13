CREATE TABLE IF NOT EXISTS users(
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(128) UNIQUE NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    hash VARCHAR(128) NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP
);

ALTER TABLE users ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE users ALTER COLUMN modified_date SET DEFAULT now();

CREATE TABLE IF NOT EXISTS games(
    game_id SERIAL PRIMARY KEY,
    string_id VARCHAR(128) UNIQUE NOT NULL,
    name VARCHAR(128) UNIQUE NOT NULL,
    description TEXT NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP
);

ALTER TABLE games ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE games ALTER COLUMN modified_date SET DEFAULT now();

CREATE TABLE IF NOT EXISTS plays(
    play_id SERIAL PRIMARY KEY,
    game_id integer REFERENCES games (game_id) NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP
);

ALTER TABLE plays ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE plays ALTER COLUMN modified_date SET DEFAULT now();

CREATE TABLE IF NOT EXISTS users_plays(
    user_id integer REFERENCES users(user_id) NOT NULL,
    play_id integer REFERENCES plays(play_id) NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP,
    PRIMARY KEY (user_id, play_id)
);

ALTER TABLE users_plays ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE users_plays ALTER COLUMN modified_date SET DEFAULT now();

CREATE TABLE IF NOT EXISTS games_users_pools(
    game_id integer REFERENCES games(game_id) NOT NULL,
    user_id integer REFERENCES users(user_id) NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP,
    PRIMARY KEY (user_id, game_id)
);

ALTER TABLE games_users_pools ALTER COLUMN created_date SET DEFAULT now();
ALTER TABLE games_users_pools ALTER COLUMN modified_date SET DEFAULT now();

CREATE TABLE IF NOT EXISTS users_games(
    user_id integer REFERENCES users(user_id) NOT NULL,
    game_id integer REFERENCES games(game_id) NOT NULL,
    score integer NOT NULL,
    created_date TIMESTAMP,
    modified_date TIMESTAMP,
    PRIMARY KEY (user_id, game_id)
);

ALTER TABLE users_games ALTER COLUMN modified_date SET DEFAULT now();
ALTER TABLE users_games ALTER COLUMN created_date SET DEFAULT now();

