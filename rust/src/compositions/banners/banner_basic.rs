use crate::compositions::banners::banner_enums::BannerResult;
use crate::compositions::UpdateDataOfComposition;

#[derive(Debug)]
pub struct BannerBasicCreateReq {}
pub struct BannerBasicRes {}

pub fn get_public(composition_source_id: u128) -> BannerResult<BannerBasicRes> {
    todo!()
}

pub fn get_private(composition_source_id: u128, author_id: u128) -> BannerResult<BannerBasicRes> {
    todo!()
}

pub fn create(create_request: &BannerBasicCreateReq, layout_id: u128, author_id: u128) -> BannerResult<BannerBasicRes> {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> BannerResult<BannerBasicRes> {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> BannerResult<BannerBasicRes> {
    todo!()
}
