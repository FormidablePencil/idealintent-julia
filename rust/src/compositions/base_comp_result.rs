pub enum BaseCompResult<Content, SuccessCode, FailureCode> {
    // success composition and source id
    Content(Content),
    // success. Source id and comp id
    Id(u128, u128),
    // success but only with a code
    SuccessCode(SuccessCode),
    Failed(FailureCode),
}
