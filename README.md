[![Rust](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml/badge.svg)](https://github.com/Acapair/acapair_follow_ban_api/actions/workflows/rust.yml)
# Acapair Follow Ban API

>: means they are variable.

## Exposed URLs
Alive Ping: "/"

Create User: "/create/:username"

Delete User: "/delete/:username"

Search User By Username: "/search-username/:username"

Search User By ID: "/search-id/:id"

Change Username: "/change-username/:username/:updated_username

Follow User: "/follow/:follower/:followed"

Unfollow User: "/unfollow/:follower/:followed"

Ban User: "/ban/:victim/:judge"

Unban User: "/unban/:victim/:judge"

Is Follower: "/is-follower/:follower/:follower"

Is Banned: "/is-banned/:victim/:judge"