use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User<T> {
    pub id: Option<T>,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl<T> User<T> {
    pub fn new(name: String, email: String) -> Self {
        let created_at = Local::now().naive_local();
        Self {
            id: None,
            name,
            email,
            created_at,
        }
    }
}
