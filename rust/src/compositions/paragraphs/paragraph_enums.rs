use strum_macros::{EnumIter, EnumString};
use crate::compositions::base_comp_result::BaseCompResult;

use crate::compositions::texts::text_basic::TextBasicRes;

#[derive(Debug, EnumIter, EnumString)]
pub enum TextType {
    Basic,
}

pub enum TextSuccessCode {}

pub enum TextFailureCode {}

pub type TextResult<Response> = BaseCompResult<Response, TextSuccessCode, TextFailureCode>;

pub enum TextResponse {
    Basic(TextResult<TextBasicRes>),
}