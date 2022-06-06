use crate::compositions::banners::banner_enums::BannerType;
use crate::compositions::carousels::carousel_enums::CarouselType;
use crate::compositions::CompositionCategory;

pub fn get_composition_res_enum(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => "CarouselBasicRes".to_string(),
            CarouselType::BlurredOverlay => "CarouselBlurredOverlayRes".to_string(),
            CarouselType::Images => "CarouselOfImagesRes".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "BannerRes".to_string(),
        },
        CompositionCategory::Text(_) => todo!(),
    }
}
