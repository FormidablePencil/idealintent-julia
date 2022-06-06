use codegen::Scope;

use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_composition_paths::get_composition_create_request_path;
use crate::gencode::gen_managers::helpers::iterate_over_all_enums_of_composition_category::iterate_over_all_enums_of_composition_category;

pub fn import_composition_models(scope: &mut Scope, composition_category: &CompositionCategory) {
    iterate_over_all_enums_of_composition_category(
        composition_category,
        &mut |comp_category: CompositionCategory| {
            let (path, create_request) = get_composition_create_request_path(&comp_category);
            scope.import(path.as_str(), create_request.as_str());
        },
    );
}