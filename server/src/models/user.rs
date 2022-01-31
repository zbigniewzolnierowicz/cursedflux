use crate::diesel::*;
use crate::schema::*;
use crate::DB;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewUserPayload {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Insertable,
    Identifiable,
    Associations,
    AsChangeset,
)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserChangeset {
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub email: String,
}

impl User {
    pub fn create(db: &DB, item: &UserChangeset) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;
        insert_into(users)
            .values(item)
            .get_result::<User>(db)
    }

    pub fn get_all(db: &DB) -> QueryResult<Vec<Self>> {
        use crate::schema::users::dsl::*;
        users.load(db)
    }
}
