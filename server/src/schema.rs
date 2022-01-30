table! {
    users (id) {
        id -> Varchar,
        username -> Text,
        password_hash -> Text,
        password_salt -> Text,
    }
}
