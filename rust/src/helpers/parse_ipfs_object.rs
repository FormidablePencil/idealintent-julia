/*
Iterates through Vec and pushes non 0 (empty) bytes to an Vec<Vec<u8>>. Where another cluster
begins start pushing bytes to a new Vec<u8>.
 */
use std::str::{from_utf8, Utf8Error};

use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataWrapper {
    pub data_type: String,
    pub metadata: String,
    pub data: String,
}

/*
Parses data from ipfs object and returns the json data of the provided type of <T>.
 */
pub fn get_data(ipfs_object: Vec<u8>) -> Option<DataWrapper> {
    let parsed_object = parse_ipfs_object(ipfs_object);

    match serde_json::from_slice::<DataWrapper>(
        parsed_object[&parsed_object.len() - 1].as_slice()
    ) {
        Ok(a) => Some(a),
        Err(_) => {
            error!("Failed to parse provided ipfs object"); // todo - error handling
            None
        }
    }
}

fn parse_ipfs_object(vec: Vec<u8>) -> Vec<Vec<u8>> {
    let mut data_clusters: Vec<Vec<u8>> = vec![];

    let mut previous_byte_empty_or_first_value = true;
    let mut cluster: Vec<u8> = vec![];

    for item in vec {
        if item != 0 {
            cluster.push(item);
            previous_byte_empty_or_first_value = false;
        } else {
            if previous_byte_empty_or_first_value == false {
                data_clusters.push(cluster);
                cluster = vec![];
            }
            previous_byte_empty_or_first_value = true;
        }
    }

    data_clusters
}

//
// pub fn get_data_as_sting(ipfs_object: Vec<u8>) -> Result<String, Utf8Error>
//     where
//         T: DeserializeOwned,
// {
//     let parsed_object = parse_ipfs_object(ipfs_object);
//
//     match from_utf8(
//         parsed_object[&parsed_object.len() - 1].as_slice()
//     ) {
//         Ok(data) => Ok(String::from(data)),
//         Err(e) => Err(e)
//     }
// }

#[cfg(test)]
mod parse_ipfs_object {
    use std::str::from_utf8;

    use futures::TryStreamExt;
    use ipfs_api_backend_actix::{IpfsApi, IpfsClient};

    use crate::helpers::parse_ipfs_object::{DataWrapper, get_data, parse_ipfs_object};
    use crate::temp_smart_contract_address_maps::crud_text;
    use crate::temp_smart_contract_address_maps::crud_text::BasicText;

    fn req() -> DataWrapper {
        DataWrapper {
            data_type: "".to_string(), // todo
            metadata: "".to_string(), // todo
            data: serde_json::to_string(&BasicText {
                title: String::from("title df df df"),
                body: String::from("body 00 0  df"),
            }).unwrap(),
        }
    }

    #[actix_rt::test]
    async fn get_ipfs_content_test() {
        let address = crud_text::upload(
            &req(),
            None,
        ).await;
        let data_bytes = IpfsClient::default()
            .get(address.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await.unwrap();
        let res = get_data(data_bytes).unwrap();

        assert_eq!(serde_json::to_string(&req()).unwrap(), serde_json::to_string(&res).unwrap());
    }

    /*
    Uploaded content to IPFS. Fetched content by address.
     */
    #[actix_rt::test]
    async fn parse_ipfs_object_test() {
        let address = crud_text::upload(
            &req(),
            None,
        ).await;
        let data_bytes = IpfsClient::default()
            .get(address.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await.unwrap();

        let data_parsed = parse_ipfs_object(data_bytes);

        // for cluster in data_parsed {
        //     println!("{:?}", from_utf8(cluster.as_slice()).unwrap());
        // };

        assert_eq!(
            format!("{}", serde_json::to_string(&req()).unwrap()),
            format!("{}", serde_json::to_string(data_parsed[data_parsed.len() - 1].as_slice()).unwrap())
        );

        assert_eq!(
            format!("{:?}", address),
            format!("{:?}", from_utf8(data_parsed[0].as_slice()).unwrap())
        );
    }
}