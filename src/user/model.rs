use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use chrono::prelude::Utc;
use chrono::NaiveDateTime;
use argonautica::Hasher;
use argonautica::Error;
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name="users"]
pub struct UpdatedUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub modified: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserData {
    pub fn to_new_user(self) -> Result<NewUser, Error> {
        let password = hash_password(self.password)?;
        let time = Utc::now().naive_utc();

        let user = NewUser {
            email: self.email,
            password: password,
            first_name: self.first_name,
            last_name: self.last_name,
            created: time,
            modified: time
        };

        Ok(user)
    }

    pub fn to_updated_user(self) -> Result<UpdatedUser, Error> {
        let password = hash_password(self.password)?;

        let user = UpdatedUser {
            email: self.email,
            password: password,
            first_name: self.first_name,
            last_name: self.last_name,
            modified: Utc::now().naive_utc()
        };

        Ok(user)
    }
}

fn hash_password(password: String) -> Result<String, Error> {
    dotenv().ok();

    let hash = Hasher::default()
        .configure_password_clearing(true)
        .configure_secret_key_clearing(true)
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY").unwrap())
        .hash()?;

    Ok(hash)
}
