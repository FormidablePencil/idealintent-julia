use strum::IntoEnumIterator;

use crate::compositions::{carousels::carousel_enums::CarouselType, CompositionCategory};
use crate::compositions::banners::banner_enums::BannerType;
use crate::compositions::texts::text_enums::TextType;

pub fn iterate_over_all_enums_of_composition_category(
    composition_category: &CompositionCategory,
    callback: &mut dyn FnMut(CompositionCategory),
) {
    match composition_category {
        CompositionCategory::Carousel(_) => {
            for item in CarouselType::iter() {
                callback(CompositionCategory::Carousel(item));
            }
        }
        CompositionCategory::Banner(_) => {
            for item in BannerType::iter() {
                callback(CompositionCategory::Banner(item));
            }
        }
        CompositionCategory::Text(_) => {
            for item in TextType::iter() {
                callback(CompositionCategory::Text(item));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use codegen::Scope;

    use crate::compositions::carousels::carousel_enums::CarouselType;
    use crate::compositions::CompositionCategory;
    use crate::gencode::gen_managers::helpers::get_composition_paths;
    use crate::gencode::gen_managers::helpers::get_composition_metadata::get_composition_metadata;
    use crate::gencode::gen_managers::import_dependencies::import_composition_models::import_composition_models;

    #[test]
    fn get_composition_create_request_test() {
        let res =
            get_composition_paths::get_composition_create_request_path(&CompositionCategory::Carousel(CarouselType::Basic));
        print!("{}, {}", res.0, res.1);
    }

    #[test]
    fn get_composition_metadata_test() {
        let res = get_composition_metadata(&CompositionCategory::Carousel(CarouselType::Basic));
        assert_eq!(
            format!("{}, {}", res.0, res.1),
            "carousel_basic, CarouselBasicCreateReq"
        );
    }

    #[test]
    fn import_composition_models_test() {
        let mut scope = Scope::new();

        import_composition_models(
            &mut scope,
            &CompositionCategory::Carousel(CarouselType::Basic),
        );

        print!("{}", scope.to_string());
    }
}