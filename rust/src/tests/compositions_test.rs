#[cfg(test)]
mod compositions_test {
    use crate::account;
    use crate::account::Account;
    use crate::compositions::{CategoryManager, CategoryResponse, CompositionCategory, ICategoryManager};
    use crate::compositions::banners::manager::BannerManager;
    use crate::compositions::carousels::carousel_basic::CarouselBasicRes;
    use crate::compositions::carousels::carousel_enums::{CarouselResponse, CarouselResult};
    use crate::compositions::carousels::manager::CarouselManager;
    use crate::compositions::paragraphs::manager::TextManager;
    use crate::compositions::paragraphs::paragraph_enums::ParagraphType;
    use crate::space::layout;

    #[test]
    fn get_new_composition() {
        let category_manager = CategoryManager { // todo - dependency inject instead
            text_manager: TextManager,
            carousel_manager: CarouselManager,
            banner_manager: BannerManager,
        };
        let composition_type = &CompositionCategory::Paragraph(ParagraphType::Basic);
        let carousel_basic = CarouselBasicRes {};

        // region setup todo - move to a reusable user flow
        let account = &Account { username: String::from("FluffyBunny") };
        let author_id = account::create(account);  // todo - create account if not created. Clean up after done.
        let layout_id= layout::create(author_id);
        // endregion setup

        // region create
        let res = category_manager.create(
            composition_type,
            Box::new(carousel_basic),
            layout_id,
            author_id,
        );

        let (source_id, comp_id) = match res {
            CategoryResponse::Carousel(comp_type) => match comp_type {
                CarouselResponse::Basic(data) => match data {
                    CarouselResult::Id(source_id, comp_id) =>
                        (source_id, comp_id),
                    _ => panic!("failed to return a success with an id"),
                }
                _ => panic!("invalid composition type returned")
            },
            _ => panic!("invalid composition category returned")
        };
        // endregion create

        // region get
        let data = match category_manager.get_public(
            composition_type,
            source_id,
        ) {
            CategoryResponse::Carousel(data) => match data {
                CarouselResponse::Basic(data) => match data {
                    CarouselResult::Content(data) => data,
                    _ => panic!("failed to get content")
                }
                _ => panic!("fetched the wrong composition type")
            },
            _ => panic!("fetched the wrong composition category")
        };

        // assert_eq!(data, carousel_basic);
        // endregion get
    }

    // #[test]
    // fn
}