use chrono::{naive::NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tribe<T, U> {
    pub id: Option<T>,
    pub name: String,
    pub leader: Option<U>,
    pub created_at: NaiveDateTime,
}

impl<T, U> Tribe<T, U> {
    pub fn new(name: String, leader: Option<U>) -> Self {
        let created_at = Local::now().naive_local();
        Self {
            id: None,
            name,
            leader,
            created_at,
        }
    }
}
