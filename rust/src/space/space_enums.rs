use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpaceRes {
    // pub space_address: u128,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpaceCreateReq {
    pub title: String,
}
