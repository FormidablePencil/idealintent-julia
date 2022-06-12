extern crate proc_macro;

use std::any::Any;

use carousels::carousel_enums::CarouselResponse;
use paragraphs::paragraph_enums::{ParagraphResponse, ParagraphType};

use crate::compositions::banners::banner_enums::BannerResponse;
use crate::compositions::carousels::manager::CarouselManager;

use self::{
    banners::{
        banner_enums::BannerType,
        manager::BannerManager,
    },
    carousels::carousel_enums::CarouselType,
    manager_impl::CompositionTypeManager,
    paragraphs::manager::TextManager,
};

pub mod banners;
pub mod carousels;
pub mod manager_impl;
pub mod paragraphs;
mod base_comp_result;
mod crud;
mod composition_relationships;
mod composition_index;

// region enum and structs
pub struct UpdateDataOfComposition {
    update_data_of: u128,
    record_update: RecordUpdate,
}

pub struct RecordUpdate {
    record_id: u128,
    update_to: Vec<UpdateColumn>,
}

pub struct UpdateColumn {
    column: u128,
    value: String,
}

pub struct CategoryManager {
    pub(crate) text_manager: TextManager,
    pub(crate) carousel_manager: CarouselManager,
    pub(crate) banner_manager: BannerManager,
}

pub enum CompositionCategory {
    Carousel(CarouselType),
    Banner(BannerType),
    Paragraph(ParagraphType),
}

pub enum CategoryResponse {
    Carousel(CarouselResponse),
    Banner(BannerResponse),
    Paragraph(ParagraphResponse),
}

// endregion enum and structs

pub trait ICategoryManager {
    fn get_public(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
    ) -> CategoryResponse;

    fn get_private(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> CategoryResponse;

    fn create(
        &self,
        comp_category: &CompositionCategory,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> CategoryResponse;

    fn update(
        &self,
        comp_category: &CompositionCategory,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    );

    fn delete(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool;
}

impl ICategoryManager for CategoryManager {
    fn get_public(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .get_public(comp_type, composition_source_id)
            ),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .get_public(comp_type, composition_source_id),
            ),
            CompositionCategory::Paragraph(comp_type) => CategoryResponse::Paragraph(
                self.text_manager
                    .get_public(comp_type, composition_source_id),
            ),
        }
    }

    fn get_private(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
            CompositionCategory::Paragraph(comp_type) => CategoryResponse::Paragraph(
                self.text_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
        }
    }

    fn create(
        &self,
        comp_category: &CompositionCategory,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .create(comp_type, create_request, layout_id, author_id),
            ),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .create(comp_type, create_request, layout_id, author_id),
            ),
            CompositionCategory::Paragraph(comp_type) => CategoryResponse::Paragraph(
                self.text_manager
                    .create(comp_type, create_request, layout_id, author_id),
            ),
        }
    }

    fn update(
        &self,
        comp_category: &CompositionCategory,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) {}

    fn delete(
        &self,
        comp_category: &CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool {
        todo!()
    }
}