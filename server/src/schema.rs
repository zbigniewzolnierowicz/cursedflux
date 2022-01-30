table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
        email -> Varchar,
    }
}
