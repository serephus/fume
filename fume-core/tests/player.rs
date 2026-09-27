use fume_core::{Endpoint, player::get_steam_level::GetSteamLevel};

#[test]
fn get_steam_level_decode() {
    let content = std::fs::read("./tests/responses/get_steam_level.json").unwrap();
    let level = GetSteamLevel::decode(&content).unwrap();
    assert_eq!(level, 17);
}
