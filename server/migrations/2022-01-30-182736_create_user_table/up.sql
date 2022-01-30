-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    id SHORTKEY PRIMARY KEY,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL
);

CREATE TRIGGER trigger_test_genid BEFORE INSERT ON users FOR EACH ROW EXECUTE PROCEDURE shortkey_generate();