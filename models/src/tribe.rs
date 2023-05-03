use crate::thing::{IdOrUser, Thing};
use chrono::{naive::NaiveDateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tribe<T>
where
    T: IdOrUser,
{
    pub id: Option<Thing>,
    pub name: String,
    pub leader: Option<T>,
    pub created_at: NaiveDateTime,
}

impl<T: IdOrUser> Tribe<T> {
    pub fn new(name: String, leader: Option<T>) -> Self {
        let created_at = Local::now().naive_local();
        Self {
            id: None,
            name,
            leader,
            created_at,
        }
    }
}
