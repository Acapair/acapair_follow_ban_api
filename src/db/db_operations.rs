use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::Channel;

use super::db_utils::*;

pub async fn connect() -> Option<Surreal<Client>> {
    establish_connection().await
}

pub async fn create(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    create_channel(username, db).await.pop().unwrap()
}

pub async fn search(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    search_channel_by_username(username, db).await
}

pub async fn delete(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    // delete channel should be last for mind sake
    // first artifacts
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
    // match delete_channel(username, db).await {
    //     Some(deleted_channel) => {
    //         match remove_follower_artifacts(deleted_channel.clone(), db).await {
    //             Some(_) => match remove_banned_artifacts(deleted_channel.clone(), db).await {
    //                 Some(_) => Some(deleted_channel),
    //                 None => None,
    //             },
    //             None => None,
    //         }
    //     }
    //     None => None,
    // }
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
