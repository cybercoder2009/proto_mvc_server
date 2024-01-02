use serde::Serialize;

#[derive(Serialize)]
pub struct Register {
    pub name: String,
}

#[derive(Serialize)]
pub struct Login {
    pub name: String,
}