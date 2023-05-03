use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::thing::{IdOrTribe, Thing};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskListStatus {
    Draft,
    InProgress,
    Completed,
    Closed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList<T>
where
    T: IdOrTribe,
{
    pub id: Option<Thing>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub due_at: Option<NaiveDateTime>,
    pub status: TaskListStatus,
    pub tribe: T,
}
