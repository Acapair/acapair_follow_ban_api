use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Id,
    Surreal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    id: Id,
    pub username: String,
    pub follower_list: Vec<Id>,
    pub banned_list: Vec<Id>,
    pub followed_list: Vec<Id>,
    pub banned_from_list: Vec<Id>,
}

pub async fn establish_connection() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();

    db.use_ns("test").use_db("test").await.unwrap();
    db
}

pub async fn search_channel_by_username(
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    let searched: Option<Channel> = db.select(("channel", username)).await.unwrap();
    searched
}
async fn search_channel_by_id(id: &Id, db: &Surreal<Client>) -> Option<Channel> {
    let searced: Option<Channel> = db.select(("channel", id.clone())).await.unwrap();
    searced
}

pub async fn create_channel(username: &String, db: &Surreal<Client>) -> Option<Vec<Channel>> {
    match search_channel_by_username(username, db).await {
        Some(_) => {
            eprintln!("Already Exists");
            return None;
        }
        None => {
            let created: Vec<Channel> = db
                .create("channel")
                .content(Channel {
                    id: Id::uuid(),
                    username: username.to_string(),
                    follower_list: vec![],
                    banned_list: vec![],
                    followed_list: vec![],
                    banned_from_list: vec![],
                })
                .await
                .unwrap();
            return Some(created);
        }
    }
}

pub async fn delete_channel(username: &String, db: &Surreal<Client>) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(channel) => {
            let deleted: Result<Option<Channel>, surrealdb::Error> =
                db.delete(("channel", channel.username)).await;
            match deleted {
                Ok(channel) => match channel {
                    Some(channel) => {
                        remove_follower_artifacts(channel.clone(), db).await;
                        remove_banned_artifacts(channel, db).await
                    }
                    None => {
                        eprintln!("Error: Channel Not Exists");
                        None
                    }
                },
                Err(err_val) => {
                    eprintln!("Error: Delete | {}", err_val);
                    None
                }
            }
        }
        None => {
            eprintln!("Not Exists");
            None
        }
    }
}
async fn update_channel(channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    match db
        .update(("channel", channel.id.clone()))
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
async fn add_id_to_vector(id: Id, mut data: Vec<Id>) -> Option<Vec<Id>> {
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
async fn remove_id_from_vector(id: Id, mut data: Vec<Id>) -> Option<Vec<Id>> {
    data.sort();
    match data.binary_search(&id) {
        Ok(_) => {
            data.retain(|_id| *_id == id);
            Some(data)
        }
        Err(_) => {
            eprintln!("Error: Not Contains");
            None
        }
    }
}
async fn remove_follower_artifacts(mut channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.follower_list.clone() {
        match search_channel_by_id(&id, db).await {
            Some(follower_channel) => {
                match remove_id_from_vector(id.clone(), follower_channel.followed_list).await {
                    Some(_) => {}
                    None => {}
                }
            }
            None => {
                eprintln!("Error: No Follower Channel by ID");
            }
        }
        channel.follower_list.retain(|_id| *_id == id);
    }
    Some(channel)
}
async fn remove_banned_artifacts(mut channel: Channel, db: &Surreal<Client>) -> Option<Channel> {
    for id in channel.banned_list.clone() {
        match search_channel_by_id(&id, db).await {
            Some(banned_channel) => {
                match remove_id_from_vector(id.clone(), banned_channel.banned_from_list).await {
                    Some(_) => {}
                    None => {}
                }
            }
            None => {
                eprintln!("Error: No Banned Channel by ID");
            }
        }
        channel.banned_list.retain(|_id| *_id == id);
    }
    Some(channel)
}

pub async fn update_channel_username(
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
pub async fn add_follower(
    follower: &String,
    username: &String,
    db: &Surreal<Client>,
) -> Option<Channel> {
    match search_channel_by_username(username, db).await {
        Some(mut channel) => match search_channel_by_username(follower, db).await {
            Some(follower) => match add_id_to_vector(follower.id, channel.follower_list).await {
                Some(follower_list) => {
                    channel.follower_list = follower_list;
                    update_channel(channel, db).await
                }
                None => {
                    eprintln!("Error: Add Follower Id");
                    None
                }
            },
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
                match remove_id_from_vector(follower.id, channel.follower_list).await {
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
            Some(banned) => match add_id_to_vector(banned.id, channel.banned_list).await {
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
            Some(banned) => match remove_id_from_vector(banned.id, channel.banned_list).await {
                Some(banned_list) => {
                    channel.banned_list = banned_list;
                    update_channel(channel, db).await
                }
                None => {
                    eprintln!("Error: Remove Banned Id");
                    None
                }
            },
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
            Some(followed) => match add_id_to_vector(followed.id, channel.followed_list).await {
                Some(followed_list) => {
                    channel.followed_list = followed_list;
                    update_channel(channel, db).await
                }
                None => {
                    eprintln!("Error: Add Followed Id");
                    None
                }
            },
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
                match remove_id_from_vector(followed.id, channel.followed_list).await {
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
                match add_id_to_vector(banned_from.id, channel.banned_from_list).await {
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
                match remove_id_from_vector(banned_from.id, channel.banned_from_list).await {
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
