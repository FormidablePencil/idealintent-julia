use std::str::{from_utf8, Utf8Error};

use actix_rt::Runtime;
use anyhow::{anyhow, Ok, Result};
use flutter_rust_bridge::ZeroCopyBuffer;
use futures::executor::block_on;
use ipfs_api_backend_actix::Error;
use crate::compositions::banners::banner_enums::BannerType;
use crate::compositions::carousels::carousel_enums::CarouselType;
use crate::compositions::CompositionCategory;
use crate::compositions::paragraphs::paragraph_enums::ParagraphType;

use crate::helpers::parse_ipfs_object::DataWrapper;
use crate::temp_smart_contract_address_maps::crud_paragraph;
use crate::temp_smart_contract_address_maps::crud_paragraph::BasicParagraph;

use flutter_rust_bridge::*;

pub fn test_success_2() -> String {
    String::from("dfd")
}