use fume_core::{
    Endpoint,
    util::{get_server_info::GetServerInfo, get_supported_api_list::GetSupportedApiList},
};

#[test]
fn get_supported_apis_decode() {
    let content = std::fs::read("./tests/responses/get_supported_apis.json").unwrap();
    let interfaces = GetSupportedApiList::decode(&content).unwrap();
    assert!(!interfaces.is_empty());
}

#[test]
fn get_server_info_decode() {
    let content = std::fs::read("./tests/responses/get_server_info.json").unwrap();
    let info = GetServerInfo::decode(&content).unwrap();
    assert_eq!(info.time_string, "Mon Jun 16 21:21:19 2025");
}
