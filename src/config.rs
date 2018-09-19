use std::env;

// 設定データ.
pub struct Config {}

impl Config {
    pub fn is_mac() -> bool {
        // 環境変数が設定されている場合、mac.
        match env::var("TARGET") {
            Ok(_) => true,
            _ => false,
        }
    }
}
