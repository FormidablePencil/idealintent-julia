use std::any::Any;
use crate::compositions::manager_impl::CompositionTypeManager;
use crate::compositions::UpdateDataOfComposition;
use super::{carousel_basic, carousel_blurred_overlay, carousel_images};
use crate::compositions::carousels::carousel_basic::CarouselBasicCreateReq;
use crate::compositions::carousels::carousel_blurred_overlay::CarouselBlurredOverlayCreateReq;
use crate::compositions::carousels::carousel_images::CarouselOfImagesCreateReq;
use crate::compositions::carousels::carousel_enums::{CarouselResponse, CarouselType};

// ==============================================================================================
//  Please don't edit this file directly. This file is auto generated.
// ==============================================================================================

pub struct CarouselManager;

impl CompositionTypeManager<CarouselType, CarouselBasicCreateReq, CarouselResponse> for CarouselManager {
    fn get_public(&self, composition_type: &CarouselType, composition_source_id: u128) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => CarouselResponse::Basic(carousel_basic::get_public(composition_source_id)),
            CarouselType::BlurredOverlay => CarouselResponse::BlurredOverlay(carousel_blurred_overlay::get_public(composition_source_id)),
            CarouselType::Images => CarouselResponse::Images(carousel_images::get_public(composition_source_id)),
        }
    }

    fn get_private(&self, composition_type: &CarouselType, composition_source_id: u128, author_id: u128) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => CarouselResponse::Basic(carousel_basic::get_private(composition_source_id, author_id)),
            CarouselType::BlurredOverlay => CarouselResponse::BlurredOverlay(carousel_blurred_overlay::get_private(composition_source_id, author_id)),
            CarouselType::Images => CarouselResponse::Images(carousel_images::get_private(composition_source_id, author_id)),
        }
    }

    fn create(&self, composition_type: &CarouselType, create_request: Box<dyn Any>, layout_id: u128, author_id: u128) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => match create_request.downcast_ref::<CarouselBasicCreateReq>() {
                Some(req) => CarouselResponse::Basic(carousel_basic::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
            CarouselType::BlurredOverlay => match create_request.downcast_ref::<CarouselBlurredOverlayCreateReq>() {
                Some(req) => CarouselResponse::BlurredOverlay(carousel_blurred_overlay::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
            CarouselType::Images => match create_request.downcast_ref::<CarouselOfImagesCreateReq>() {
                Some(req) => CarouselResponse::Images(carousel_images::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
        }
    }

    fn update(&self, composition_type: &CarouselType, composition_update_que: Vec<UpdateDataOfComposition>, composition_source_id: u128, author_id: u128) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => CarouselResponse::Basic(carousel_basic::update(composition_update_que, composition_source_id, author_id)),
            CarouselType::BlurredOverlay => CarouselResponse::BlurredOverlay(carousel_blurred_overlay::update(composition_update_que, composition_source_id, author_id)),
            CarouselType::Images => CarouselResponse::Images(carousel_images::update(composition_update_que, composition_source_id, author_id)),
        }
    }

    fn delete(&self, composition_type: &CarouselType, composition_source_id: u128, author_id: u128) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => CarouselResponse::Basic(carousel_basic::delete(composition_source_id, author_id)),
            CarouselType::BlurredOverlay => CarouselResponse::BlurredOverlay(carousel_blurred_overlay::delete(composition_source_id, author_id)),
            CarouselType::Images => CarouselResponse::Images(carousel_images::delete(composition_source_id, author_id)),
        }
    }
}