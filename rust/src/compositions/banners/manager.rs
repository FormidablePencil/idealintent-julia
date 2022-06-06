use std::any::Any;
use crate::compositions::manager_impl::CompositionTypeManager;
use crate::compositions::UpdateDataOfComposition;
use super::banner_basic;
use crate::compositions::banners::banner_basic::BannerBasicCreateReq;
use crate::compositions::banners::banner_enums::{BannerResponse, BannerType};

// ==============================================================================================
//  Please don't edit this file directly. This file is auto generated.
// ==============================================================================================

pub struct BannerManager;

impl CompositionTypeManager<BannerType, BannerBasicCreateReq, BannerResponse> for BannerManager {
    fn get_public(&self, composition_type: &BannerType, composition_source_id: u128) -> BannerResponse {
        match composition_type {
            BannerType::Basic => BannerResponse::Basic(banner_basic::get_public(composition_source_id)),
        }
    }

    fn get_private(&self, composition_type: &BannerType, composition_source_id: u128, author_id: u128) -> BannerResponse {
        match composition_type {
            BannerType::Basic => BannerResponse::Basic(banner_basic::get_private(composition_source_id, author_id)),
        }
    }

    fn create(&self, composition_type: &BannerType, create_request: Box<dyn Any>, layout_id: u128, author_id: u128) -> BannerResponse {
        match composition_type {
            BannerType::Basic => match create_request.downcast_ref::<BannerBasicCreateReq>() {
                Some(req) => BannerResponse::Basic(banner_basic::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
        }
    }

    fn update(&self, composition_type: &BannerType, composition_update_que: Vec<UpdateDataOfComposition>, composition_source_id: u128, author_id: u128) -> BannerResponse {
        match composition_type {
            BannerType::Basic => BannerResponse::Basic(banner_basic::update(composition_update_que, composition_source_id, author_id)),
        }
    }

    fn delete(&self, composition_type: &BannerType, composition_source_id: u128, author_id: u128) -> BannerResponse {
        match composition_type {
            BannerType::Basic => BannerResponse::Basic(banner_basic::delete(composition_source_id, author_id)),
        }
    }
}