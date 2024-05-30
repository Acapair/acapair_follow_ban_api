[![Rust](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml/badge.svg)](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml)
# Acapair Follow Ban API

>: means they are variable.

## Exposed URLs
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