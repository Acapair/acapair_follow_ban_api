use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, sql::Thing, Surreal};

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    username: String,
    follower_list: Vec<String>,
    banned_list: Vec<String>,
}


async fn establish_connection() -> Surreal<Client>{
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    db.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    db.use_ns("test").use_db("test").await.unwrap();
    db
}

async fn create_channel(username:&String, db:&Surreal<Client>) -> Option<Vec<Channel>>{
    match search_channel(username, db).await {
        Some(_) => {
            println!("Already Exists");
            return None;
        },
        None => {
            let created: Vec<Channel> = db.create("channel").content(Channel {
                username:username.to_string(),
                follower_list: vec![],
                banned_list: vec![],
            }).await.unwrap();
            return Some(created);
        },
    }
}

async fn delete_channel(username:&String, db:&Surreal<Client>) -> Option<Channel> {
    match search_channel(username, db).await {
        Some(channel) => {
            let deleted: Option<Channel> = db.delete(("channel", channel.username)).await.unwrap();
            return deleted;
        }
        None => {
            println!("Not Exists");
            return None;
        }
    }
}

async fn search_channel(username:&String, db:&Surreal<Client>) -> Option<Channel> {
    let searched: Option<Channel> = db.select(("channel", username)).await.unwrap();
    searched
}

async fn update_channel(){}
async fn add_follower(){}
async fn remove_follewer(){}
async fn ban_user(){}
async fn unban_user(){}


