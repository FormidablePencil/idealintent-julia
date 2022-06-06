use crate::compositions::CompositionCategory;

pub fn get_composition_index_metadata(index_address: String) {
}

pub fn get_composition(index_address: String) {
}

pub fn add(
    comp_address: String,
    comp_category: &CompositionCategory,
) {
    // save_in_collection_of_all it under the name of composition_category/composition
}

pub fn remove() {}

#[cfg(test)]
mod composition_index_test {
    #[test]
    fn add() {
        // create composition
        
        // add composition to index
        // get composition through composition_index
    }

    #[test]
    fn get_composition_index_metadata() {
    }
    
    #[test]
    fn get_composition() {
    }
    
    #[test]
    fn remove() {
        // todo - integration test since the composition needs to be removed itself as well
    }
}
