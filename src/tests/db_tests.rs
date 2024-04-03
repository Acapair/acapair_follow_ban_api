#[cfg(test)]
use crate::db::db_operations::*;
use tokio::test;

#[cfg(test)]
async fn create_connection_for_tests(
    db_name: &str,
) -> surrealdb::Surreal<surrealdb::engine::remote::ws::Client> {
    let connection = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("127.0.0.1:8000")
        .await
        .unwrap();
    connection
        .signin(surrealdb::opt::auth::Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
    connection.use_ns("test").use_db(db_name).await.unwrap();
    connection
}

#[test]
async fn test_connect() {
    assert_eq!(connect().await.is_some(), true);
}
#[test]
async fn test_create() {
    let connection = create_connection_for_tests("test_create").await;

    let name = &"Ahmet".to_string();
    let created = create(name, &connection).await;

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(created.is_some(), true);
}

#[test]
async fn test_search() {
    let connection = create_connection_for_tests("test_search").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await;
    let searched = search(name, &connection).await;

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(created, searched);
}

#[test]
async fn test_delete() {
    let connection = create_connection_for_tests("test_delete").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await;
    let deleted = delete(name, &connection).await;

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(created, deleted);
}

#[test]
async fn test_change_username() {
    let connection = create_connection_for_tests("test_change_username").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await.unwrap();
    let changed = change_username(&"Kaan".to_string(), name, &connection)
        .await
        .unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(created.id, changed.clone().id);
    assert_eq!(changed.username, "Kaan");
}

#[test]
async fn test_follow() {
    let connection = create_connection_for_tests("test_follow").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let mut follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    let mut followed = search(name_followed, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );
}

#[test]
async fn test_unfollow() {
    let connection = create_connection_for_tests("test_unfollow").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let mut follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    let mut followed = search(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    follower = unfollow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    followed = search(name_followed, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);
}

#[test]
async fn test_ban() {
    let connection = create_connection_for_tests("test_ban").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search(name_judge, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);
    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);
}

#[test]
async fn test_unban() {
    let connection = create_connection_for_tests("test_unban").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search(name_judge, &connection).await.unwrap();

    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);
    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);

    victim = unban(name_victim, name_judge, &connection).await.unwrap();
    judge = search(name_judge, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(victim.banned_from_list.pop().is_none(), true);
    assert_eq!(judge.banned_list.pop().is_none(), true);
}

#[test]
async fn test_delete_follower() {
    let connection = create_connection_for_tests("test_delete_follower").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let mut follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    let mut followed = search(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    follower = delete(name_follower, &connection).await.unwrap();
    followed = search(name_followed, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);
}

#[test]
async fn test_delete_followed() {
    let connection = create_connection_for_tests("test_delete_followed").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let mut follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    let mut followed = search(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    followed = delete(name_followed, &connection).await.unwrap();
    follower = search(name_follower, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);
}

#[test]
async fn test_delete_victim() {
    let connection = create_connection_for_tests("test_delete_victim").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection)
        .await
        .unwrap();
    let mut judge = search(name_judge, &connection).await.unwrap();

    assert_eq!(
        judge.banned_list.pop().unwrap(),
        victim.id.unwrap().id
    );
    assert_eq!(
        victim.banned_from_list.pop().unwrap(),
        judge.id.unwrap().id
    );

    victim = delete(name_victim, &connection).await.unwrap();
    judge = search(name_judge, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(judge.banned_list.pop().is_none(), true);
    assert_eq!(victim.banned_from_list.pop().is_none(), true);
}

#[test]
async fn test_delete_judge() {
    let connection = create_connection_for_tests("test_delete_judge").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection)
        .await
        .unwrap();
    let mut judge = search(name_judge, &connection).await.unwrap();

    assert_eq!(
        judge.banned_list.pop().unwrap(),
        victim.id.unwrap().id
    );
    assert_eq!(
        victim.banned_from_list.pop().unwrap(),
        judge.id.unwrap().id
    );

    judge = delete(name_judge, &connection).await.unwrap();
    victim = search(name_victim, &connection).await.unwrap();

    let _cleaning = connection.query("DELETE channel;").await;
    assert_eq!(judge.banned_list.pop().is_none(), true);
    assert_eq!(victim.banned_from_list.pop().is_none(), true);
}
