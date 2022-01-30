-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    id SHORTKEY PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    password_salt VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);

CREATE TRIGGER trigger_users_genid BEFORE INSERT ON users FOR EACH ROW EXECUTE PROCEDURE shortkey_generate();