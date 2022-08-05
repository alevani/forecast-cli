use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeRegistrationBody {
    pub person: i32,
    pub task: i32,
    pub time_registered: i32,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestError {
    pub status: i32,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TimeRegistrationResponse {
    pub status_code: StatusCode,
    pub recipient_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeRegistrationsPerProjectResponse {
    pub id: i32,
    pub task: Option<i32>,
    pub project: Option<i32>,
    pub notes: Option<String>,
    pub date: Option<String>,
    pub time_registered: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub company_task_id: i32,
    pub project_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}
