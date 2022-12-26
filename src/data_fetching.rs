use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub identity_number: String,
    pub email: String,
}

pub fn read_db() -> Result<Vec<Ticket>, Error> {
    let db_content = fs::read_to_string("./data/db.json")?;
    let parsed: Vec<Ticket> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn get_user_data(id: i32) -> Option<User> {
    let res = reqwest::blocking::get(format!("127.0.0.1:1207/user/{}", id)).unwrap();
    match res.json::<User>() {
        Ok(parsed) => Some(parsed),
        Err(_) => None,
    }
}
