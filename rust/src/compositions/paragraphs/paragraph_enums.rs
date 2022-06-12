use strum_macros::{EnumIter, EnumString};
use crate::compositions::base_comp_result::BaseCompResult;

use crate::compositions::paragraphs::paragraph_basic::ParagraphBasicRes;

#[derive(Debug, EnumIter, EnumString)]
pub enum ParagraphType {
    Basic,
}

pub enum ParagraphSuccessCode {}

pub enum ParagraphFailureCode {}

pub type ParagraphResult<Response> = BaseCompResult<Response, ParagraphSuccessCode, ParagraphFailureCode>;

pub enum ParagraphResponse {
    Basic(ParagraphResult<ParagraphBasicRes>),
}