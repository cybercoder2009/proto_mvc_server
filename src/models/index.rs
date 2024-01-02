use serde::Serialize;

#[derive(Serialize)]
pub struct Record {
    pub id: String,
    pub title: String,
}