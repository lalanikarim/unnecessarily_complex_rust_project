use serde::{Deserialize, Serialize};

use crate::thing::{IdOrUser, Thing};

#[derive(Serialize, Deserialize, Debug)]
pub enum TodoStatus<T>
where
    T: IdOrUser,
{
    Open,
    Assigned(T),
    Completed(T),
    Closed(T),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo<T>
where
    T: IdOrUser,
{
    pub id: Option<Thing>,
    pub details: String,
    pub status: TodoStatus<T>,
    pub evidence: Option<String>,
}

impl<T: IdOrUser> Todo<T> {
    pub fn new(details: String) -> Self {
        Self {
            id: None,
            details,
            status: TodoStatus::Open,
            evidence: None,
        }
    }
}
