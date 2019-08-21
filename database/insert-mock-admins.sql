INSERT INTO users (username, email, password_hash, is_game_designer, is_admin)
VALUES
    ('admin', 'admin@gaime.org', '$2y$12$9RPZU9/bD.QlzG8gh2UxruYDF2pEK4O92qGsUqO1wuYJovCDE0RTu', TRUE, TRUE),
    ('gamedesigner', 'gamedesigner@gaime.org', '$2y$12$9RPZU9/bD.QlzG8gh2UxruYDF2pEK4O92qGsUqO1wuYJovCDE0RTu', TRUE, FALSE);
