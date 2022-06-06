use std::fs::File;
use std::io::{Cursor, Write};
use std::str::from_utf8;

use flutter_rust_bridge::handler::Error;
use futures::TryStreamExt;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};

use crate::helpers::parse_ipfs_object::{DataWrapper, get_data};
use crate::temp_smart_contract_address_maps::encryption;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicText {
    pub title: String,
    pub body: String,
}

pub(crate) async fn upload(content: &DataWrapper, secret_encryption_key: Option<String>) -> String {
    let serialized_data = serde_json::to_string(content).unwrap();
    let data = match secret_encryption_key {
        None => {
            Cursor::new(serialized_data.as_bytes().to_owned())
        }
        Some(secret_key) => {
            let encrypted_data = encryption::encrypt(secret_key.as_bytes(), &serialized_data);
            Cursor::new(encrypted_data.as_slice().to_owned())
        }
    };

    let client = IpfsClient::default();
    let address = match client.add(data).await {
        Ok(res) => res.hash,
        Err(e) => panic!("error adding file: {:?}", e) // todo - return Result instead perhaps
    };

    // pin content through Storj ipfs
    let pinned = false;

    // save_in_collection_of_all address to private smart contract
    // crud_address_smart_contract::save_in_collection_of_all(address, pinned);

    address
}

pub(crate) async fn get(address: &str) -> Result<Option<DataWrapper>, ipfs_api_backend_actix::Error> {
    match IpfsClient::default()
        .get(address)
        .map_ok(|chunk| {
            chunk.to_vec()
        })
        .try_concat()
        .await
    {
        Ok(res) => {
            Ok(get_data(res))
        }
        Err(e) => Err(e)
    }
}

fn update(address: String) {}

fn delete(address: String) {}

#[cfg(test)]
mod smart_contract_address_maps_tests {
    use std::str::from_utf8;

    use crate::{helpers::parse_ipfs_object::DataWrapper, temp_smart_contract_address_maps::crud_text::{BasicText, get, upload}};

    #[actix_rt::test]
    async fn crud() {

        // let ipfs_content_address = String::from("ipfs_content_address");
        // save_in_collection_of_all(ipfs_content_address);

        let address = upload(&DataWrapper {
            data_type: "".to_string(), // todo
            metadata: "".to_string(), // todo
            data: serde_json::to_string(&BasicText {
                title: String::from("title df df df"),
                body: String::from("body 00 0  df"),
            }).unwrap(),
        },
                             // Some(String::from("my very secret key 123 abcdefghi")),
                             None,
        ).await;
        // println!("{address}");

        let res = get(address.as_str()).await.unwrap();
        // let utf = from_utf8(res.as_slice());
        // println!("{}", utf.unwrap());
        //
        // // println!("{:?}", res.unwrap());
        // let f:BasicText = serde_json::from_slice(res.as_slice()).unwrap();
        // println!("{:?}", f);
    }
}