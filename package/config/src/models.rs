use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub runtime: Runtime,
    pub jwt: Jwt,
    pub database: Database,
    pub shutdown_signal: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Runtime {
    pub addr: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Jwt {
    pub public: String,
    pub private: String,
    pub expire: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DatabaseEngine {
    Speedb,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    pub engine: DatabaseEngine,
    pub file: String,
    pub namespace: String,
    pub database: String,
}
