use strum_macros::{EnumIter, EnumString};

use crate::compositions::base_comp_result::BaseCompResult;
use crate::compositions::carousels::carousel_basic::CarouselBasicRes;
use crate::compositions::carousels::carousel_blurred_overlay::CarouselBlurredOverlayRes;
use crate::compositions::carousels::carousel_images::CarouselOfImagesRes;

#[derive(Debug, EnumIter, EnumString)]
pub enum CarouselType {
    Basic,
    BlurredOverlay,
    Images,
}

pub enum CarouselSuccessCode {}

pub enum CarouselFailureCode {}

pub type CarouselResult<Response> = BaseCompResult<Response, CarouselSuccessCode, CarouselFailureCode>;

pub enum CarouselResponse {
    Basic(CarouselResult<CarouselBasicRes>),
    BlurredOverlay(CarouselResult<CarouselBlurredOverlayRes>),
    Images(CarouselResult<CarouselOfImagesRes>),
}
