use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{Channel, DataBaseConfig};

use super::db_utils::*;

pub async fn connect(database_config: &DataBaseConfig) -> Option<Surreal<Client>> {
    establish_connection(
        &database_config.address,
        &database_config.username,
        &database_config.password,
        &database_config.namespace,
        &database_config.database,
    )
    .await
}

pub async fn create(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    match create_channel(username, db).await.pop() {
        Some(channel) => channel,
        None => None,
    }
}

pub async fn search_username(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    search_channel_by_username(username, db).await
}

pub async fn search_id(id: &String, db: &Surreal<Client>) -> Option<Channel> {
    search_channel_by_id(&id.into(), db).await
}

pub async fn delete(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(channel) => match remove_all_followers(channel.clone(), db).await {
            Some(_) => match remove_all_followed(channel.clone(), db).await {
                Some(_) => match remove_all_banned(channel.clone(), db).await {
                    Some(_) => match remove_all_banned_from(channel, db).await {
                        Some(_) => delete_channel(username, db).await,
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        },
        None => {
            eprintln!("Error: Delete | Channel Not Exists");
            None
        }
    }
}

pub async fn change_username(
    updated_username: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => {
            channel.username = updated_username.to_string();
            update_channel(channel, db).await
        }
        None => {
            eprintln!("Error: Update Username");
            None
        }
    }
}

pub async fn follow(follower: &String, followed: &String, db: &Surreal<Client>) -> Option<Channel> {
    match add_follower(follower, followed, db).await {
        Some(_) => add_followed(followed, follower, db).await,
        None => None,
    }
}

pub async fn unfollow(
    follower: &String,
    followed: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match remove_follower(follower, followed, db).await {
        Some(_) => remove_followed(followed, follower, db).await,
        None => None,
    }
}

pub async fn ban(victim: &String, judge: &String, db: &Surreal<Client>) -> Option<Channel> {
    match add_banned(victim, judge, db).await {
        Some(_) => add_banned_from(judge, victim, db).await,
        None => None,
    }
}

pub async fn unban(victim: &String, judge: &String, db: &Surreal<Client>) -> Option<Channel> {
    match remove_banned(victim, judge, db).await {
        Some(_) => remove_banned_from(judge, victim, db).await,
        None => None,
    }
}

pub async fn is_follower(follower: &String, followed: &String, db: &Surreal<Client>) -> bool {
    is_follower_by_username(follower, followed, db).await
}

pub async fn is_banned(victim: &String, judge: &String, db: &Surreal<Client>) -> bool {
    is_banned_by_username(victim, judge, db).await
}
