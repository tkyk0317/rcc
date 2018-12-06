use std::env;
use std::process::Command;

// 設定データ.
pub struct Config {}

impl Config {
    pub fn is_mac() -> bool {
        // macで動作しているかチェック.
        let uname = Command::new("uname").output().expect("uname is error");
        match String::from_utf8_lossy(&uname.stdout).find("Darwin") {
            Some(_) => true,
            _ => false,
        }
    }
}
