use std::any::Any;
use crate::compositions::manager_impl::CompositionTypeManager;
use crate::compositions::UpdateDataOfComposition;
use super::text_basic;
use crate::compositions::texts::text_basic::TextBasicCreateReq;
use crate::compositions::texts::text_enums::{TextResponse, TextType};

// ==============================================================================================
//  Please don't edit this file directly. This file is auto generated.
// ==============================================================================================

pub struct TextManager;

impl CompositionTypeManager<TextType, TextBasicCreateReq, TextResponse> for TextManager {
    fn get_public(&self, composition_type: &TextType, composition_source_id: u128) -> TextResponse {
        match composition_type {
            TextType::Basic => TextResponse::Basic(text_basic::get_public(composition_source_id)),
        }
    }

    fn get_private(&self, composition_type: &TextType, composition_source_id: u128, author_id: u128) -> TextResponse {
        match composition_type {
            TextType::Basic => TextResponse::Basic(text_basic::get_private(composition_source_id, author_id)),
        }
    }

    fn create(&self, composition_type: &TextType, create_request: Box<dyn Any>, layout_id: u128, author_id: u128) -> TextResponse {
        match composition_type {
            TextType::Basic => match create_request.downcast_ref::<TextBasicCreateReq>() {
                Some(req) => TextResponse::Basic(text_basic::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
        }
    }

    fn update(&self, composition_type: &TextType, composition_update_que: Vec<UpdateDataOfComposition>, composition_source_id: u128, author_id: u128) -> TextResponse {
        match composition_type {
            TextType::Basic => TextResponse::Basic(text_basic::update(composition_update_que, composition_source_id, author_id)),
        }
    }

    fn delete(&self, composition_type: &TextType, composition_source_id: u128, author_id: u128) -> TextResponse {
        match composition_type {
            TextType::Basic => TextResponse::Basic(text_basic::delete(composition_source_id, author_id)),
        }
    }
}