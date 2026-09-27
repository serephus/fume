use fume_core::{
    Endpoint, Relationship, SteamId,
    user::{
        get_friend_list::GetFriendList, get_player_summaries::GetPlayerSummaries,
        get_user_group_list::GetUserGroupList, resolve_vanity_url::ResolveVanityUrl,
    },
};

#[test]
fn get_friend_list_decode() {
    let content = std::fs::read("./tests/responses/get_friend_list.json").unwrap();
    let friends = GetFriendList::decode(&content).unwrap();
    assert!(!friends.is_empty());
    assert_eq!(friends[0].relationship, Relationship::Friend);
}

#[test]
fn get_user_group_list_decode() {
    let content = std::fs::read("./tests/responses/get_user_group_list.json").unwrap();
    let groups = GetUserGroupList::decode(&content).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].get(), 4_755_837);
}

#[test]
fn resolve_vanity_url_success_decode() {
    let content = std::fs::read("./tests/responses/resolve_vanity_url_success.json").unwrap();
    let steamid = ResolveVanityUrl::decode(&content).unwrap();
    assert_eq!(steamid, Some(SteamId(76_561_198_084_913_741)));
}

#[test]
fn resolve_vanity_url_failure_decode() {
    let content = std::fs::read("./tests/responses/resolve_vanity_url_failure.json").unwrap();
    let steamid = ResolveVanityUrl::decode(&content).unwrap();
    assert_eq!(steamid, None);
}

#[test]
fn get_player_summaries_decode() {
    let content = std::fs::read("./tests/responses/get_player_summaries.json").unwrap();
    let players = GetPlayerSummaries::decode(&content).unwrap();
    assert_eq!(players.len(), 1);
    assert_eq!(players[0].persona_name, "philippe.stroobandt");
}
