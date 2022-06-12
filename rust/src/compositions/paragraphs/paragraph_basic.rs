use crate::compositions::paragraphs::paragraph_enums::ParagraphResult;
use crate::compositions::UpdateDataOfComposition;

pub struct ParagraphBasicCreateReq {}
pub struct ParagraphBasicRes {}

pub fn get_public(composition_source_id: u128) -> ParagraphResult<ParagraphBasicRes> {
    todo!()
}

pub fn get_private(composition_source_id: u128, author_id: u128) -> ParagraphResult<ParagraphBasicRes> {
    todo!()
}

pub fn create(
    create_request: &ParagraphBasicCreateReq,
    layout_id: u128,
    author_id: u128,
) -> ParagraphResult<ParagraphBasicRes> {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> ParagraphResult<ParagraphBasicRes> {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> ParagraphResult<ParagraphBasicRes> {
    todo!()
}
