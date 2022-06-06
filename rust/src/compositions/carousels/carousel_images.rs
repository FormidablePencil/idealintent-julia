use crate::compositions::carousels::carousel_enums::CarouselResult;
use crate::compositions::UpdateDataOfComposition;

pub struct CarouselOfImagesCreateReq {}

pub struct CarouselOfImagesRes {}

pub fn get_public(composition_source_id: u128) -> CarouselResult<CarouselOfImagesRes> {
    todo!()
}

pub fn get_private(composition_source_id: u128, author_id: u128) -> CarouselResult<CarouselOfImagesRes> {
    todo!()
}

pub fn create(
    create_request: &CarouselOfImagesCreateReq,
    layout_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselOfImagesRes> {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselOfImagesRes> {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> CarouselResult<CarouselOfImagesRes> {
    todo!()
}
