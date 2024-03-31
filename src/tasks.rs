use std::path::PathBuf;

use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub text:String, 
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>
}
impl Task {
    pub fn new(text:String) -> Task{
        return Task { text, created_at: Utc::now()}
    }
}

pub fn add_task(route_file:PathBuf, task:Task){
    todo!()
}

pub fn complete_task(route_file:PathBuf, task_position:usize){
    todo!()
}

pub fn get_tasks(route_file:PathBuf){
    todo!()
}