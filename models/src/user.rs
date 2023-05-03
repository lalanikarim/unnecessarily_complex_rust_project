use chrono::{Local, NaiveDateTime};

use crate::thing::{IdOrUser, Thing};

pub trait IsUser {}

pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl IdOrUser for User {}

impl User {
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
