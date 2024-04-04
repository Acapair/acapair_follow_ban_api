use crate::Channel;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Id,
    Surreal,
};

use super::db_operations::{unban, unfollow};
pub async fn establish_connection() -> Option<Surreal<Client>> {
    match Surreal::new::<Ws>("127.0.0.1:8000").await {
        Ok(db) => {
            match db
                .signin(Root {
                    username: "root",
                    password: "root",
                })
                .await
            {
                Ok(_) => match db.use_ns("test").use_db("test").await {
                    Ok(_) => Some(db),
                    Err(err_val) => {
                        eprintln!("Error: DB Use | {}", err_val);
                        None
                    }
                },
                Err(err_val) => {
                    eprintln!("Error: DB Login | {}", err_val);
                    None
                }
            }
        }
        Err(err_val) => {
            eprintln!("Error: DB Connection | {}", err_val);
            None
        }
    }
}

async fn search_channel_by_id(id: &Id, db: &Surreal<Client>) -> Option<Channel> {
    let searced: Option<Channel> = db.select(("channel", id.clone())).await.unwrap();
    searced
}
fn id_extractor(channel: &Channel) -> Id {
    match channel.id.clone() {
        Some(thing) => thing.id,
        None => {
            eprintln!("Error: Thing Not Exists");
            channel.id.clone().unwrap().id
        }
    }
}

pub async fn search_channel_by_username(
    username: &String,
    db: &Surreal<Client>,
) -> Option<crate::Channel> {
    let searched: Vec<Option<Channel>> = db.select("channel").await.unwrap();
    for element in searched {
        match element {
            Some(channel) => {
                if channel.username == username.to_string() {
                    return Some(channel);
                }
            }
            None => {
                eprintln!("No Content");
            }
        }
    }
    None
}

pub async fn create_channel(username: &String, db: &Surreal<Client>) -> Vec<Option<Channel>> {
    match search_channel_by_username(username, db).await {
        Some(_) => {
            eprintln!("Already Exists");
            return vec![];
        }
        None => {
            db.query("DEFINE INDEX usernameINDEX ON TABLE channel COLUMNS username UNIQUE")
                .await
                .unwrap();
            let created: Vec<Option<Channel>> = db
                .create("channel")
                .content(Channel {
                    id: None,
                    username: username.to_string(),
                    follower_list: vec![],
                    banned_list: vec![],
                    followed_list: vec![],
                    banned_from_list: vec![],
                })
                .await
                .unwrap();
            created
        }
    }
}

pub async fn delete_channel(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(channel) => db
            .delete(("channel", id_extractor(&channel)))
            .await
            .unwrap(),
        None => {
            eprintln!("Not Exists");
            None
        }
    }
}

pub async fn update_channel(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    match db
        .update(("channel", channel.clone().id.unwrap()))
        .content(Channel {
            id: channel.id,
            username: channel.username,
            follower_list: channel.follower_list,
            banned_list: channel.banned_list,
            followed_list: channel.followed_list,
            banned_from_list: channel.banned_from_list,
        })
        .await
    {
        Ok(option_channel) => match option_channel {
            Some(channel) => channel,
            None => {
                eprintln!("Channel Does Not Exists");
                None
            }
        },
        Err(err_val) => {
            eprintln!("Update Failed: {}", err_val);
            None
        }
    }
}
fn add_id_to_vector(id: Id, mut data: Vec<Id>) -> Option<Vec<Id>> {
    data.sort();
    match data.binary_search(&id) {
        Ok(_) => {
            eprintln!("Error: Already Contains");
            None
        }
        Err(_) => {
            data.push(id);
            Some(data)
        }
    }
}
fn remove_id_from_vector(id: Id, mut data: Vec<Id>) -> Option<Vec<Id>> {
    data.sort();
    match data.binary_search(&id) {
        Ok(_) => {
            data.retain(|_id| *_id != id);
            Some(data)
        }
        Err(_) => {
            eprintln!("Error: Not Contains");
            None
        }
    }
}
pub async fn add_follower(
    follower: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(follower, db).await {
            Some(follower) => {
                match add_id_to_vector(id_extractor(&follower), channel.follower_list) {
                    Some(follower_list) => {
                        channel.follower_list = follower_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Add Follower Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Follower Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Add Follower");
            None
        }
    }
}
pub async fn remove_follower(
    follower: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(follower, db).await {
            Some(follower) => {
                match remove_id_from_vector(id_extractor(&follower), channel.follower_list) {
                    Some(follower_list) => {
                        channel.follower_list = follower_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Remove Follower Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Follower Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Remove Follower");
            None
        }
    }
}
pub async fn add_banned(
    banned: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(banned, db).await {
            Some(banned) => match add_id_to_vector(id_extractor(&banned), channel.banned_list) {
                Some(banned_list) => {
                    channel.banned_list = banned_list;
                    update_channel(channel, db).await
                }
                None => {
                    eprintln!("Error: Add Banned Id");
                    None
                }
            },
            None => {
                eprintln!("Error: Banned Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Add Banned");
            None
        }
    }
}
pub async fn remove_banned(
    banned: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(banned, db).await {
            Some(banned) => {
                match remove_id_from_vector(id_extractor(&banned), channel.banned_list) {
                    Some(banned_list) => {
                        channel.banned_list = banned_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Remove Banned Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Banned Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Remove Banned");
            None
        }
    }
}
pub async fn add_followed(
    followed: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(followed, db).await {
            Some(followed) => {
                match add_id_to_vector(id_extractor(&followed), channel.followed_list) {
                    Some(followed_list) => {
                        channel.followed_list = followed_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Add Followed Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Followed Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Add Followed");
            None
        }
    }
}
pub async fn remove_followed(
    followed: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(followed, db).await {
            Some(followed) => {
                match remove_id_from_vector(id_extractor(&followed), channel.followed_list) {
                    Some(followed_list) => {
                        channel.followed_list = followed_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Remove Followed Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Followed Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Remove Followed");
            None
        }
    }
}
pub async fn add_banned_from(
    banned_from: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(banned_from, db).await {
            Some(banned_from) => {
                match add_id_to_vector(id_extractor(&banned_from), channel.banned_from_list) {
                    Some(banned_from_list) => {
                        channel.banned_from_list = banned_from_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Add Banned from Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Followed Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Add Banned From");
            None
        }
    }
}
pub async fn remove_banned_from(
    banned_from: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(banned_from, db).await {
            Some(banned_from) => {
                match remove_id_from_vector(id_extractor(&banned_from), channel.banned_from_list) {
                    Some(banned_from_list) => {
                        channel.banned_from_list = banned_from_list;
                        update_channel(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Remove Banned from Id");
                        None
                    }
                }
            }
            None => {
                eprintln!("Error: Banned Not Exists");
                None
            }
        },
        None => {
            eprintln!("Error: Remove Banned From");
            None
        }
    }
}

pub async fn remove_all_followers(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.follower_list {
        match search_channel_by_id(&id, db).await {
            Some(follower_channel) => {
                match unfollow(&follower_channel.username, &channel.username, db).await {
                    Some(_) => {}
                    None => {
                        eprintln!("Error: Can't Remove Follower");
                    }
                }
            }
            None => {
                eprintln!("Error: Can't Remove Follower, Follower Not Exists");
            }
        }
    }
    search_channel_by_username(&channel.username, db).await
}

pub async fn remove_all_followed(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.followed_list {
        match search_channel_by_id(&id, db).await {
            Some(followed_channel) => {
                match unfollow(&channel.username, &followed_channel.username, db).await {
                    Some(_) => {}
                    None => {
                        eprintln!("Error: Can't Remove Followed");
                    }
                }
            }
            None => {
                eprintln!("Error: Can't Remove Followed, Followed Not Exists");
            }
        }
    }
    search_channel_by_username(&channel.username, db).await
}

pub async fn remove_all_banned(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.banned_list {
        match search_channel_by_id(&id, db).await {
            Some(banned_channel) => {
                match unban(&banned_channel.username, &channel.username, db).await {
                    Some(_) => {}
                    None => {
                        eprintln!("Error: Can't Remove Banned");
                    }
                }
            }
            None => {
                eprintln!("Error: Can't Remove Banned, Banned Not Exists");
            }
        }
    }
    search_channel_by_username(&channel.username, db).await
}

pub async fn remove_all_banned_from(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.banned_from_list {
        match search_channel_by_id(&id, db).await {
            Some(banned_from_channel) => {
                match unban(&channel.username, &banned_from_channel.username, db).await {
                    Some(_) => {}
                    None => {
                        eprintln!("Error: Can't Remove Banned From");
                    }
                }
            }
            None => {
                eprintln!("Error: Can't Remove Banned From, Banned From Not Exists");
            }
        }
    }
    search_channel_by_username(&channel.username, db).await
}
