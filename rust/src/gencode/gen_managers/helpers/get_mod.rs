use crate::compositions::banners::banner_enums::BannerType;
use crate::compositions::carousels::carousel_enums::CarouselType;
use crate::compositions::CompositionCategory;
use crate::compositions::paragraphs::paragraph_enums::ParagraphType;

pub fn get_mod(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => "carousel_basic".to_string(),
            CarouselType::BlurredOverlay => "carousel_blurred_overlay".to_string(),
            CarouselType::Images => "carousel_images".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
        },
        CompositionCategory::Paragraph(comp_type) => match comp_type {
            ParagraphType::Basic => "paragraph_basic".to_string(),
        },
    }
}
