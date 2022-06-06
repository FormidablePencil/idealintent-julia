use crate::compositions::CompositionCategory;
// use crate::space::relationship_enums::RelationshipDirection;

pub fn association(
    first_address: String,
    first_composition_category: &CompositionCategory,
    second_address: String,
    second_composition_category: &CompositionCategory,
    // relationship: RelationshipDirection,
) {
    // save_in_collection_of_all it under the name of composition_category/composition

    // match composition_category {
    //     CompositionCategory::Carousel(comp_type) => {}
    //     CompositionCategory::Banner(comp_type) => {}
    //     CompositionCategory::Text(comp_type) => {}
    // }
}

pub fn remove_association(first_address: String, second_address: String) {}

#[cfg(test)]
mod composition_index_test {
    #[test]
    fn associate() {
        
    }

    #[test]
    fn remove_association() {

    }
}
