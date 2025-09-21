use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Instruction {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Trick {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize)]
pub struct TrickCreateInput {
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Serialize, Deserialize)]
pub struct TrickReplaceInput {
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}
