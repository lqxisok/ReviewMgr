use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_SQLITE_PATH: Mutex<String> = Mutex::new(String::new());
}