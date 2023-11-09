use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct GetNotesQuery {
    pub skip: Option<usize>,
    pub take: Option<usize>,
}

#[derive(Serialize, Debug)]
pub enum NoteErrorResponse {
    NotFound(String),
    AlreadyExists(String),
    BadRequest(String),
}
