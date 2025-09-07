[![Rust](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml/badge.svg)](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml)
# Acapair Follow Ban API

Container Start Config: 

>podman run --rm --net host -v ABSOLUTE_PATH_FROM_HOST:/configs:z  -v ABSOLUTE_PATH_FROM_HOST:/certificates:z localhost/acapair_follow_ban_api:latest


## Exposed URLs
>: means they are variable.

Alive Ping(get): "/"

Create User(post): "/:username"

Delete User(delete): "/:username"

Search User By Username(get): "/:username"

Search User By ID(get): "/id/:id"

Change Username(patch): "/username/:username/:updated_username

Follow User(patch): "/follow/:follower/:followed"

Unfollow User(patch): "/unfollow/:follower/:followed"

Ban User(patch): "/ban/:victim/:judge"

Unban User(patch): "/unban/:victim/:judge"

Is Follower(get): "/is-follower/:follower/:follower"

Is Banned(get): "/is-banned/:victim/:judge"