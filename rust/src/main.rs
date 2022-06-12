use lib_flutter_rust_bridge_codegen::{frb_codegen, Opts};

fn main() {
    println!("cargo:rerun-if-changed={}", "src/api.rs");
    frb_codegen(Opts {
        rust_input: "src/api.rs".to_string(),
        dart_output: "../lib/bridge_generated.dart".to_string(),
        ..Default::default()
    }
    ).unwrap();
}

// // dependency injection?
//
// // Access the models directly without the need of the hierarchy; service, manager, repository.
// //  This is because the logic doesn't have to be accessed from the central location of routes. Now the logic can be directly accessed there is no need for a hierarchy anymore.
// //  Instead of services, managers, repositories, the logic is going to be organized by composition categories.
//
// // use std::ops::Residual;
//
// use core::fmt;
// use std::error::Error;
// use std::io;
//
// use codegen::{Block, Function, Scope};
// // use ipfs_api::{IpfsApi, IpfsClient};
// use julia_cms::account::create_document;
// use julia_cms::compositions::CompositionCategory;
// use julia_cms::config::db_connection;
// use std::io::{Cursor, Write};
// use futures::TryStreamExt;
// use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
// use flutter_rust_bridge_example::account::create_document;
//
// #[macro_use]
// extern crate dotenv_codegen;
//
// #[actix_rt::main]
// async fn main() {
//     let client = IpfsClient::default();
//     let data = Cursor::new("Hello World!");
//
//     match client.add(data).await {
//         Ok(res) => {
//             // println!("{}", res.hash);
//             match client
//                 .get(res.hash.as_str())
//                 .map_ok(|chunk| chunk.to_vec())
//                 .try_concat()
//                 .await
//             {
//                 Ok(res) => {
//                     let out = io::stdout();
//                     let mut out = out.lock();
//
//                     out.write_all(&res).unwrap();
//                 }
//                 Err(e) => eprintln!("error getting file: {:?}", e)
//             }
//         },
//         Err(e) => eprintln!("error adding file: {:?}", e)
//     }
// }
//
// // use julia_cms::compositions::{
// //     banners::{banner_basic::BannerBasicCreateReq, BannerManager, BannerType},
// //     carousels::carousel_blurred_overlay::get_public,
// // };
// // std::io::Read
//
// #[tokio::main]
// async fn main33() {
//     let res = create_document().await.unwrap();
//     // match res {
//     //     Ok(_) => println!("success"),
//     //     Err(err) => panic!("{}", err)
//     // }
// }
//
// // fn main22() {
// //     let f1 = db_connection();
// //     println!("{:?}", f1);
// //     // modify
// //     {
// //         let mut conf = f1.lock().unwrap();
// //         conf.db_connection_str = "hello".to_string();
// //     }
// //
// //     let f2 = db_connection();
// //     println!("{:?}", f2);
// //     let conf2 = f2.lock().unwrap();
// //
// //     assert_eq!(conf2.db_connection_str, "hello".to_string());
// // }
//
// fn ipfs_test() {
//     // let api = IpfsApi::new("127.0.0.1", 5001);
//
//     // let data = DataTest123 {};
//     // let res = api.block_put(data);
//
//     // let bytes = api
//     //     .cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")
//     //     .unwrap();
//     // let data = String::from_utf8(bytes.collect()).unwrap();
//
//     // println!("k");
//     // println!("{}", data);
//     // println!("k");
// }
//
// fn main2() {
//     // ~ CRUD components
//
//     // impl_composition_type_manager(CompositionCategory::Banner(BannerType::Basic));
//     // impl_composition_type_manager(
//     //     CompositionCategory::Carousel(CarouselType::Basic),
//     //     "BannerBasicCreateReq",
//     // );
//     // impl_composition_type_manager(CompositionCategory::Text(TextType::Basic));
//     // let banner_basic_arm = CrudOperation::Create.get_arms(
//     //     "BannerType".to_string(),
//     //     "Basic".to_string(),
//     //     "banner_basic",
//     // );
//
//     // let mut arms_for_methods = Vec::new();
//
//     // for item in CrudOperation::iter() {
//     // arms_for_methods.push(item.get_arms("BannerType", "Basic", "banner_basic"));
//     // }
//
//     // trait BannerType2 {
//     //     fn get_name(&self) -> String {
//     //         format!("{}", &self)
//     //     }
//     // }
//
//     //     println!("Hello, world!");
//     // compositions::carousels::add();
//     //     carousel_basic::delete(1);
//
//     //     get_public(1);
// }
//
// // use std::io::Cursor;
//
// // #[tokio::main]
// // async fn main() {
// // let client = IpfsClient::default();
// // let data = Cursor::new("Hello World!");
//
// // match client.add(data).await {
// //     Ok(res) => println!("{}", res.hash),
// //     Err(e) => eprintln!("error adding file: {}", e),
// // }
// // }
