use crate::compositions::carousels::carousel_enums::CarouselResult;
use crate::compositions::UpdateDataOfComposition;

pub struct CarouselBlurredOverlayCreateReq {
    title: String,
}

pub struct CarouselBlurredOverlayRes {
    title: String,
    composition_source_id: u128,
}

pub fn get_public(composition_source_id: u128) -> CarouselResult<CarouselBlurredOverlayRes> {
    todo!()
}

pub fn get_private(
    composition_source_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselBlurredOverlayRes> {
    todo!()
}

pub fn create(
    create_request: &CarouselBlurredOverlayCreateReq,
    layout_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselBlurredOverlayRes> {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> CarouselResult<CarouselBlurredOverlayRes> {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> CarouselResult<CarouselBlurredOverlayRes> {
    todo!()
}

#[cfg(test)]
mod carousel_blurred_overlay_tests {
    use super::{CarouselBlurredOverlayCreateReq, create};

    #[test]
    fn public_crud_test() {
        crud_test(true)
    }

    #[test]
    fn private_crud_test() {
        crud_test(false)
    }

    fn crud_test(is_public: bool) {
        let data = CarouselBlurredOverlayCreateReq {
            title: String::from("test"),
        };

        let res = create(&data, 1, 1);

        // let composition_source_id: u128 = match res {
        //     Some(id) => id,
        //     None => todo!(),
        // };
        //
        // let res = if is_public == true {
        //     match get_public(composition_source_id) {
        //         Some(res) => res,
        //         None => panic!("Make this test fail"),
        //     }
        // } else {
        //     match get_public(composition_source_id) {
        //         Some(res) => res,
        //         None => panic!("Make this test fail"),
        //     }
        //     // assert!(res.is_some());
        //     // assert!(res.is_none());
        // };

        // assert_eq!(res.composition_source_id, composition_source_id);
        // assert_eq!(res.title, res.title);

        // assert_eq!(2 + 2, 4);
    }
}
