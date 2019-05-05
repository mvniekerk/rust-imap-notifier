use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Email {
    pub domain: String,
    pub port: u32,
    pub username: String,
    pub password: String,
    pub refresh: u32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Folder {
    pub folder: String,
    pub subject: String,
    pub commands: Vec<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailConfig {
    pub email: Email,
    pub folders: Vec<Folder>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    connections: Vec<EmailConfig>
}