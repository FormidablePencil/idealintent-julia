use std::any::Any;
use crate::compositions::manager_impl::CompositionTypeManager;
use crate::compositions::UpdateDataOfComposition;
use super::paragraph_basic;
use crate::compositions::paragraphs::paragraph_basic::ParagraphBasicCreateReq;
use crate::compositions::paragraphs::paragraph_enums::{ParagraphResponse, ParagraphType};

// ==============================================================================================
//  Please don't edit this file directly. This file is auto generated.
// ==============================================================================================

pub struct TextManager;

impl CompositionTypeManager<ParagraphType, ParagraphBasicCreateReq, ParagraphResponse> for TextManager {
    fn get_public(&self, composition_type: &ParagraphType, composition_source_id: u128) -> ParagraphResponse {
        match composition_type {
            ParagraphType::Basic => ParagraphResponse::Basic(paragraph_basic::get_public(composition_source_id)),
        }
    }

    fn get_private(&self, composition_type: &ParagraphType, composition_source_id: u128, author_id: u128) -> ParagraphResponse {
        match composition_type {
            ParagraphType::Basic => ParagraphResponse::Basic(paragraph_basic::get_private(composition_source_id, author_id)),
        }
    }

    fn create(&self, composition_type: &ParagraphType, create_request: Box<dyn Any>, layout_id: u128, author_id: u128) -> ParagraphResponse {
        match composition_type {
            ParagraphType::Basic => match create_request.downcast_ref::<ParagraphBasicCreateReq>() {
                Some(req) => ParagraphResponse::Basic(paragraph_basic::create(req, layout_id, author_id)),
                None => panic!("failed...")
            }
        }
    }

    fn update(&self, composition_type: &ParagraphType, composition_update_que: Vec<UpdateDataOfComposition>, composition_source_id: u128, author_id: u128) -> ParagraphResponse {
        match composition_type {
            ParagraphType::Basic => ParagraphResponse::Basic(paragraph_basic::update(composition_update_que, composition_source_id, author_id)),
        }
    }

    fn delete(&self, composition_type: &ParagraphType, composition_source_id: u128, author_id: u128) -> ParagraphResponse {
        match composition_type {
            ParagraphType::Basic => ParagraphResponse::Basic(paragraph_basic::delete(composition_source_id, author_id)),
        }
    }
}