use fume_core::{Endpoint, app::get_app_list::GetAppList};

#[test]
fn get_app_list_decode() {
    let content = std::fs::read("./tests/responses/get_app_list.json").unwrap();
    let apps = GetAppList::decode(&content).unwrap();
    assert!(!apps.is_empty());
    assert_eq!(apps[0].appid.get(), 5);
    assert_eq!(apps[0].name, "Dedicated Server");
}
