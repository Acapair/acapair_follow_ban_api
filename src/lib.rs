use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

pub mod db;
pub mod routing;
pub mod tests;

#[derive(Debug, Clone)]
pub struct AppState {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    pub id: Option<Thing>,
    pub username: String,
    pub follower_list: Vec<Id>,
    pub banned_list: Vec<Id>,
    pub followed_list: Vec<Id>,
    pub banned_from_list: Vec<Id>,
}
