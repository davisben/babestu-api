use serde::{Serialize, Deserialize};
use chrono::prelude::Utc;
use chrono::NaiveDateTime;
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
    pub fn to_new_user(self) -> NewUser {
        let time = Utc::now().naive_utc();

        NewUser {
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            created: time,
            modified: time
        }
    }

    pub fn to_updated_user(self) -> UpdatedUser {
        UpdatedUser {
            email: self.email,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            modified: Utc::now().naive_utc()
        }
    }
}
