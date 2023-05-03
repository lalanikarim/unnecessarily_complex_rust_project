use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TodoStatus<U> {
    Open,
    Assigned(U),
    Completed(U),
    Closed(U),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo<T, U> {
    pub id: Option<T>,
    pub details: String,
    pub status: TodoStatus<U>,
    pub evidence: Option<String>,
}

impl<T, U> Todo<T, U> {
    pub fn new(details: String) -> Self {
        Self {
            id: None,
            details,
            status: TodoStatus::Open,
            evidence: None,
        }
    }
}
