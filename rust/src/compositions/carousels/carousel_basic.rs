use crate::compositions::carousels::carousel_enums::CarouselResult;
use crate::compositions::UpdateDataOfComposition;

pub struct CarouselBasicCreateReq {}

pub struct CarouselBasicRes {}

pub fn get_public(composition_source_id: u128) -> CarouselResult<CarouselBasicRes> {
    todo!()
}

pub fn get_private(composition_source_id: u128, author_id: u128) -> CarouselResult<CarouselBasicRes> {
    todo!()
}

pub fn create(
    create_request: &CarouselBasicCreateReq,
    layout_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselBasicRes> {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselBasicRes> {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> CarouselResult<CarouselBasicRes> {
    todo!()
}
