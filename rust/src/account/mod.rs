use dotenv_codegen::dotenv;
use firebase_rs::Firebase;
use firestore_grpc::tonic::{
    codegen::InterceptedService,
    metadata::MetadataValue,
    Request,
    Status, transport::{Channel, ClientTlsConfig},
};
use firestore_grpc::v1::{
    CreateDocumentRequest, Document, firestore_client::FirestoreClient, Value, value::ValueType,
};
use serde::{Deserialize, Serialize};

use crate::config::firebase;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub username: String,
}

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;

const URL: &'static str = "https://julia-ce110-default-rtdb.firebaseio.com";
const DOMAIN: &'static str = "julia-ce110.firebaseapp.com";

async fn get_client() -> Result<FirestoreClient<InterceptedService<Channel, impl Fn(Request<()>) -> Result<Request<()>, Status>>>, BoxError> {
    let endpoint = Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN));

    let bearer_token = format!("Bearer {}", dotenv!("TOKEN"));
    let header_value = MetadataValue::from_str(&bearer_token)?;

    let channel = endpoint?.connect().await?;
    println!("get_client hit");

    let service = FirestoreClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut()
            .insert("authorization", header_value.clone());
        Ok(req)
    });
    Ok(service)
}


pub async fn create_document() -> Result<(), BoxError> {
    let parent = format!(
        // "projects/{}/databases/(default)/documents",
        "projects/{}/databases/(default)/test",
        dotenv!("PROJECT_ID")
    );
    let collection_id = "testme".into();
    let document_id = "".into();
    let mut fields = std::collections::HashMap::new();
    fields.insert(
        "message".into(),
        Value {
            value_type: Some(ValueType::StringValue("Hello world!".into())),
        },
    );
    let document = Some(Document {
        name: "".into(),
        fields,
        create_time: None,
        update_time: None,
    });
    let mut res = get_client()
        .await?.create_document(CreateDocumentRequest {
        parent,
        collection_id,
        document_id,
        document,
        mask: None,
    }).await?;
    println!("hit");

    // Ok(res.into_inner())
    Ok(())
}

fn firebase_account() -> Firebase {
    firebase().at("account")
}

pub fn create(request: &Account) -> u128 {
    firebase_account().set(request);
    todo!()
}

pub async fn get(address: &str) {
    firebase_account().at(address).get::<Account>();
    // match firebase_account().at(address).get::<Account>().await {
    //     Ok(data) => Ok(data),
    //     Err(err) => Err(err)
    // }
}

pub fn update(request: &Account, address: &str) {
    firebase_account().at(address).update(request);
}

#[cfg(test)]
mod account {
    use crate::account::create_document;

    #[actix_rt::test]
    async fn testing() {
        let res = create_document().await;
        match res {
            Ok(_) => println!("success"),
            Err(err) => panic!("{}", err)
        }
    }
}