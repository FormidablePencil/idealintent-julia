use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_composition_metadata::get_composition_metadata;

pub fn get_comp_path(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(_) => "crate::compositions::carousels",
        CompositionCategory::Banner(_) => "crate::compositions::banners",
        CompositionCategory::Text(_) => "crate::compositions::texts",
    }.to_string()
}

pub fn get_composition_create_request_path(
    composition_category: &CompositionCategory,
) -> (String, String) {
    fn setup_import(
        composition_category: &CompositionCategory,
    ) -> Box<dyn FnOnce(String, String) -> (String, String)> {
        fn setup(
            composition_category: &CompositionCategory,
        ) -> Box<dyn FnOnce(String, String) -> (String, String)> {
            let setup_path = get_comp_path(&composition_category);

            Box::new(move |path: String, request: String| {
                (format!("{setup_path}::{path}"), String::from(request))
            })
        }

        match composition_category {
            CompositionCategory::Carousel(_) => setup(&composition_category),
            CompositionCategory::Banner(_) => setup(&composition_category),
            CompositionCategory::Text(_) => setup(&composition_category),
        }
    }

    let (first, second) = get_composition_metadata(&composition_category);
    let import = setup_import(&composition_category);
    import(first, second)
}
