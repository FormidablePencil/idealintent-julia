use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use dotenv_codegen::dotenv;
// use dotenv::dotenv;
use serde::{Deserialize};

use firebase_rs::*;

#[derive(Debug)]
pub struct Config {
    pub db_connection_str: String,
    pub firebase: Firebase,
}

// todo - deprecated
pub fn db_connection() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: "test config".to_string(),
            firebase: Firebase::auth(
                format!("https://{}", dotenv!("dbUrl")).as_str(),
                dotenv!("dbKey")
            ).unwrap()
            // Firebase::new("https://myfirebase.firebaseio.com").unwrap(),
        }));
    });

    unsafe { &*CONF.as_ptr() }
}

#[derive(Deserialize, Debug)]
pub struct AddressResponse {
    pub name: String
}

pub fn firebase() -> Firebase {
    Firebase::auth(
        format!("https://{}", dotenv!("dbUrl")).as_str(),
    dotenv!("dbKey")
    ).unwrap()
}