extern crate serde_json;

use firebase_rs::Firebase;

use crate::config::{AddressResponse, firebase};
use crate::space::space_enums::{SpaceCreateReq, SpaceRes};

pub fn firebase_space() -> Firebase {
    firebase().at("spaces.json")
}

pub async fn get(space_address: &str) -> Result<SpaceRes, String> {
    match firebase_space().at(space_address).get::<SpaceRes>().await {
        Ok(msg) => Ok(msg),
        Err(error) => Err(String::from(error.to_string()))
    }
}

pub async fn create(space: &SpaceCreateReq) -> Result<AddressResponse, String> {
    match firebase_space().set(space).await {
        Ok(res) => Ok(serde_json::from_str(res.data.as_str()).unwrap()),
        Err(err) => Err(format!("{:?}", err))
    }
}

pub async fn update(space: &SpaceCreateReq, space_address: &str) -> Result<(), String> {
    match firebase_space().at(space_address).update(space).await {
        Ok(res) => Ok(()),
        Err(err) => Err(format!("{:?}", err))
    }
}

pub async fn delete(space_address: &str) -> Result<(), String> {
    match firebase_space().at(space_address).delete().await {
        Ok(res) => Ok(()),
        Err(err) => Err(format!("{:?}", err))
    }
}

#[cfg(test)]
mod compositions_arrangement {
    use crate::space::compositions_arrangement::{create, delete, get, update};
    use crate::space::space_enums::SpaceCreateReq;

    #[actix_rt::test]
    async fn space_crud() {
        let req = &SpaceCreateReq { title: "a title".to_string() };

        let space_address = create(req).await.unwrap().name;

        assert_eq!(get(&space_address).await.unwrap().title, req.title);

        let updated_req = &SpaceCreateReq { title: "new title".to_string() };
        let update_res = update(updated_req, &space_address).await;
        assert!(update_res.is_ok());
        assert_eq!(get(&space_address).await.unwrap().title, updated_req.title);

        assert_eq!(delete(space_address.as_str()).await.ok().unwrap(), ());

        assert!(get(&space_address).await.is_err());
    }
}