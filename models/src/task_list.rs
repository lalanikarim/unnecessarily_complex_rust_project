use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskListStatus {
    Draft,
    InProgress,
    Completed,
    Closed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList<T, TT> {
    pub id: Option<T>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub due_at: Option<NaiveDateTime>,
    pub status: TaskListStatus,
    pub tribe: TT,
}
