use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum CompositionTypeManagerMethod {
    GetPublic,
    GetPrivate,
    Create,
    Update,
    Delete,
}

impl CompositionTypeManagerMethod {
    pub(crate) fn get_method_name(&self) -> String {
        match self {
            CompositionTypeManagerMethod::GetPublic => "get_public".to_string(),
            CompositionTypeManagerMethod::GetPrivate => "get_private".to_string(),
            CompositionTypeManagerMethod::Create => "create".to_string(),
            CompositionTypeManagerMethod::Update => "update".to_string(),
            CompositionTypeManagerMethod::Delete => "delete".to_string(),
        }
    }

    pub(crate) fn get_args_for_method(&self) -> String {
        match self {
            CompositionTypeManagerMethod::GetPublic => "(composition_source_id)".to_string(),
            CompositionTypeManagerMethod::GetPrivate => "(composition_source_id, author_id)".to_string(),
            CompositionTypeManagerMethod::Create => "(create_request, layout_id, author_id)".to_string(),
            CompositionTypeManagerMethod::Update => "(composition_update_que, composition_source_id, author_id)".to_string(),
            CompositionTypeManagerMethod::Delete => "(composition_source_id, author_id)".to_string(),
        }
    }
}
