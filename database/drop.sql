ALTER TABLE games DROP CONSTRAINT games_user_id_fkey;
ALTER TABLE plays_results DROP CONSTRAINT plays_results_play_id_fkey;
ALTER TABLE plays_results DROP CONSTRAINT plays_results_user_id_fkey;
ALTER TABLE users_in_plays DROP CONSTRAINT users_in_plays_user_id_fkey;
ALTER TABLE users_waiting_for_games DROP CONSTRAINT users_waiting_for_games_user_id_fkey;
ALTER TABLE users_ratings DROP CONSTRAINT users_ratings_user_id_fkey;
ALTER TABLE plays DROP CONSTRAINT plays_game_id_fkey;
ALTER TABLE users_waiting_for_games DROP CONSTRAINT users_waiting_for_games_game_id_fkey;
ALTER TABLE users_ratings DROP CONSTRAINT users_ratings_game_id_fkey;
ALTER TABLE users_in_plays DROP CONSTRAINT users_in_plays_play_id_fkey;

DROP TABLE if exists users cascade;
DROP TABLE if exists games;
DROP TABLE if exists plays;
DROP TABLE if exists plays_results;
DROP TABLE if exists users_in_plays;
DROP TABLE if exists users_waiting_for_games;
DROP TABLE if exists users_ratings;
