use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

pub mod db;
pub mod routing;
pub mod tests;
pub mod utils;

#[derive(Debug, Clone)]
pub struct DataBaseConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Surreal<Client>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub id: Option<Thing>,
    pub username: String,
    pub follower_list: Vec<Id>,
    pub banned_list: Vec<Id>,
    pub followed_list: Vec<Id>,
    pub banned_from_list: Vec<Id>,
}
