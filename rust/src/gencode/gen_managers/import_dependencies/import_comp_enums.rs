use codegen::Scope;

use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;
use crate::gencode::gen_managers::helpers::get_composition_paths::get_comp_path;
use crate::get_composition_name;

pub fn import_comp_enums(scope: &mut Scope, composition_category: &CompositionCategory) {
    let get_comp_enum_path = || {
        format!(
            "{}::{}_enums",
            get_comp_path(composition_category),
            get_composition_name!(composition_category).to_lowercase()
        )
    };

    scope.import(
        &*get_comp_enum_path(),
        format!("{}Response", get_composition_name!(composition_category)).as_str(),
    );

    scope.import(
        &*get_comp_enum_path(),
        &*get_composition_name(&composition_category, true),
    );
}


