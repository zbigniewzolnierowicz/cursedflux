use crate::diesel::*;
use crate::schema::*;
use crate::DB;
use argon2::Config;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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

impl Into<UserChangeset> for NewUserPayload {
    fn into(self) -> UserChangeset {
        let Self { username, email, .. } = self;
        let password_salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let password_hash = argon2::hash_encoded(
            self.password.as_ref(),
            password_salt.as_ref(),
            &Config::default(),
        )
            .unwrap();
        UserChangeset {
            username,
            email,
            password_hash,
            password_salt,
        }
    }
}

impl Into<UserChangeset> for &NewUserPayload {
    fn into(self) -> UserChangeset {
        self.clone().into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct UserPublic {
    pub username: String,
    pub email: String,
}

impl Into<UserPublic> for User {
    fn into(self) -> UserPublic {
        self.get_public()
    }
}

impl Into<UserPublic> for &User {
    fn into(self) -> UserPublic {
        self.clone().get_public()
    }
}

impl User {
    pub fn create(db: &DB, item: &NewUserPayload) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;
        let new_user: UserChangeset = item.into();
        insert_into(users)
            .values(
                new_user
            )
            .get_result::<User>(db.clone())
    }

    pub fn get_all(db: &DB) -> QueryResult<Vec<Self>> {
        use crate::schema::users::dsl::*;
        users.load(db)
    }

    pub fn get_public(self) -> UserPublic {
        let Self { email, username, .. } = self;
        UserPublic {
            username,
            email
        }
    }
}
