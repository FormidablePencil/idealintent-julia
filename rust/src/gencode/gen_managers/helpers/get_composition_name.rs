use crate::compositions::CompositionCategory;

pub fn get_composition_name(composition_category: &CompositionCategory, is_type: bool) -> String {
    match composition_category {
        CompositionCategory::Carousel(_) => "Carousel",
        CompositionCategory::Banner(_) => "Banner",
        CompositionCategory::Text(_) => "Text",
    }
    .to_string()
        + if is_type == true { "Type" } else { "" }
}

#[macro_export]
macro_rules! get_composition_name {
    ($composition_category: expr) => {
        get_composition_name($composition_category, false)
    };
}
