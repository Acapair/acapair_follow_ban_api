#[cfg(test)]
use crate::db::db_operations::*;
use tokio::test;

#[cfg(test)]
async fn create_connection_for_tests(
    db_name: &str,
) -> surrealdb::Surreal<surrealdb::engine::remote::ws::Client> {
    let connection = surrealdb::Surreal::new::<surrealdb::engine::remote::ws::Ws>("127.0.0.1:5000")
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
    assert_eq!(
        create_connection_for_tests("test_connect")
            .await
            .health()
            .await
            .is_ok(),
        true
    );
}
#[test]
async fn test_create() {
    let connection = create_connection_for_tests("test_create").await;

    let name = &"Ahmet".to_string();
    let created = create(name, &connection).await;

    assert_eq!(created.is_some(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_search_username() {
    let connection = create_connection_for_tests("test_search_username").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await;
    let searched = search_username(name, &connection).await;

    assert_eq!(created, searched);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_search_id() {
    let connection = create_connection_for_tests("test_search_username").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await;
    let id = &created.clone().unwrap().id.unwrap().id.to_string();
    let searched = search_id(id, &connection).await;

    assert_eq!(created, searched);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_delete() {
    let connection = create_connection_for_tests("test_delete").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await;
    let deleted = delete(name, &connection).await;

    assert_eq!(created, deleted);
    assert_eq!(search_username(name, &connection).await.is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_change_username() {
    let connection = create_connection_for_tests("test_change_username").await;
    let name = &"Ahmet".to_string();

    let created = create(name, &connection).await.unwrap();
    let changed = change_username(&"Kaan".to_string(), name, &connection)
        .await
        .unwrap();

    assert_eq!(created.id, changed.clone().id);
    assert_eq!(changed.username, "Kaan");

    let _cleaning = connection.query("DELETE channel;").await;
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
    let mut followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    let _cleaning = connection.query("DELETE channel;").await;
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
    let mut followed = search_username(name_followed, &connection).await.unwrap();

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
    followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_ban() {
    let connection = create_connection_for_tests("test_ban").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);
    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_unban() {
    let connection = create_connection_for_tests("test_unban").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);
    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);

    victim = unban(name_victim, name_judge, &connection).await.unwrap();
    judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(victim.banned_from_list.pop().is_none(), true);
    assert_eq!(judge.banned_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
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
    let mut followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    follower = delete(name_follower, &connection).await.unwrap();
    followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
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
    let mut followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );

    followed = delete(name_followed, &connection).await.unwrap();
    follower = search_username(name_follower, &connection).await.unwrap();

    assert_eq!(followed.follower_list.pop().is_none(), true);
    assert_eq!(follower.followed_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_delete_victim() {
    let connection = create_connection_for_tests("test_delete_victim").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);
    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);

    victim = delete(name_victim, &connection).await.unwrap();
    judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(judge.banned_list.pop().is_none(), true);
    assert_eq!(victim.banned_from_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_delete_judge() {
    let connection = create_connection_for_tests("test_delete_judge").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);
    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);

    judge = delete(name_judge, &connection).await.unwrap();
    victim = search_username(name_victim, &connection).await.unwrap();

    assert_eq!(judge.banned_list.pop().is_none(), true);
    assert_eq!(victim.banned_from_list.pop().is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_follow_already_follower() {
    let connection = create_connection_for_tests("test_follow_already_follower").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let mut follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();
    let mut followed = search_username(name_followed, &connection).await.unwrap();

    assert_eq!(
        followed.follower_list.pop().unwrap(),
        follower.id.unwrap().id
    );
    assert_eq!(
        follower.followed_list.pop().unwrap(),
        followed.id.unwrap().id
    );
    assert_eq!(
        follow(name_follower, name_followed, &connection)
            .await
            .is_none(),
        true
    );
    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_unfollow_already_nonfollower() {
    let connection = create_connection_for_tests("test_unfollow_already_nonfollower").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    assert_eq!(
        unfollow(name_follower, name_followed, &connection)
            .await
            .is_none(),
        true
    );
    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_ban_already_banned() {
    let connection = create_connection_for_tests("test_ban_already_banned").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let mut victim = ban(name_victim, name_judge, &connection).await.unwrap();
    let mut judge = search_username(name_judge, &connection).await.unwrap();

    assert_eq!(victim.banned_from_list.pop().unwrap(), judge.id.unwrap().id);
    assert_eq!(judge.banned_list.pop().unwrap(), victim.id.unwrap().id);
    assert_eq!(
        ban(name_victim, name_judge, &connection).await.is_none(),
        true
    );

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_unban_already_nonbanned() {
    let connection = create_connection_for_tests("test_unban_already_nonbanned").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    assert_eq!(
        unban(name_victim, name_judge, &connection).await.is_none(),
        true
    );

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_delete_noncreated() {
    let connection = create_connection_for_tests("test_delete_noncreated").await;
    let name = &"Ahmet".to_string();

    assert_eq!(delete(name, &connection).await.is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_create_already_created() {
    let connection = create_connection_for_tests("test_create_already_created").await;

    let name = &"Ahmet".to_string();
    let created = create(name, &connection).await;

    assert_eq!(created.is_some(), true);
    assert_eq!(create(name, &connection).await.is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_search_username_noncreated() {
    let connection = create_connection_for_tests("test_search_username_noncreated").await;
    let name = &"Ahmet".to_string();

    assert_eq!(search_username(name, &connection).await.is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_search_id_noncreated() {
    let connection = create_connection_for_tests("test_search_id_noncreated").await;
    let name = &"Ahmet".to_string();

    assert_eq!(search_id(name, &connection).await.is_none(), true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_is_follower_already_follower() {
    let connection = create_connection_for_tests("test_is_follower_already_follower").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    let _follower = follow(name_follower, name_followed, &connection)
        .await
        .unwrap();

    assert_eq!(
        is_follower(name_follower, name_followed, &connection).await,
        true
    );

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_is_follower_nonfollower() {
    let connection = create_connection_for_tests("test_is_follower_nonfollower").await;
    let name_follower = &"Ahmet".to_string();
    let name_followed = &"Kaan".to_string();

    let _follower = create(name_follower, &connection).await.unwrap();
    let _followed = create(name_followed, &connection).await.unwrap();

    assert_eq!(
        is_follower(name_follower, name_followed, &connection).await,
        false
    );

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_is_banned_already_banned() {
    let connection = create_connection_for_tests("test_is_banned_already_banned").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    let _victim = ban(name_victim, name_judge, &connection).await.unwrap();

    assert_eq!(is_banned(name_victim, name_judge, &connection).await, true);

    let _cleaning = connection.query("DELETE channel;").await;
}

#[test]
async fn test_is_banned_nonbanned() {
    let connection = create_connection_for_tests("test_is_banned_nonbanned").await;
    let name_victim = &"Ahmet".to_string();
    let name_judge = &"Kaan".to_string();

    let _victim = create(name_victim, &connection).await.unwrap();
    let _judge = create(name_judge, &connection).await.unwrap();

    assert_eq!(is_banned(name_victim, name_judge, &connection).await, false);

    let _cleaning = connection.query("DELETE channel;").await;
}
