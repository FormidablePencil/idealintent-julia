use strum_macros::{EnumIter, EnumString};

use crate::compositions::banners::banner_basic::BannerBasicRes;
use crate::compositions::base_comp_result::BaseCompResult;

#[derive(Debug, EnumIter, EnumString)]
pub enum BannerType {
    Basic,
}

pub enum BannerSuccessCode {}

pub enum BannerFailureCode {}

pub type BannerResult<Response> = BaseCompResult<Response, BannerSuccessCode, BannerFailureCode>;

pub enum BannerResponse {
    Basic(BannerResult<BannerBasicRes>),
}