use std::any::Any;

use super::UpdateDataOfComposition;

pub trait CompositionTypeManager<CompositionType, CreateRequest, Response> {
    fn get_public(&self, comp_type: &CompositionType, composition_source_id: u128) -> Response;

    fn get_private(
        &self,
        comp_type: &CompositionType,
        composition_source_id: u128,
        author_id: u128,
    ) -> Response;

    fn create(
        &self,
        comp_type: &CompositionType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> Response;

    fn update(
        &self,
        comp_type: &CompositionType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) -> Response;

    fn delete(
        &self,
        comp_type: &CompositionType,
        composition_source_id: u128,
        author_id: u128,
    ) -> Response;
}
