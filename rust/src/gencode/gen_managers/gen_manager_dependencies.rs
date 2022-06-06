use codegen::Scope;

use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;
use crate::gencode::gen_managers::import_dependencies::import_comp_enums::import_comp_enums;
use crate::gencode::gen_managers::import_dependencies::import_composition_models::import_composition_models;
use crate::gencode::gen_managers::import_dependencies::import_mods::import_composition_mods;

pub fn gen_manager_dependencies(scope: &mut Scope, composition_category: &CompositionCategory) {
    scope.import("std::any", "Any");
    scope.import(
        "crate::compositions::manager_impl",
        "CompositionTypeManager",
    );
    scope.import("crate::compositions", "UpdateDataOfComposition");
    import_composition_mods(scope, &composition_category);
    import_composition_models(scope, &composition_category);
    import_composition_models(scope, composition_category);
    import_comp_enums(scope, composition_category);

    let manager_struct = format!(
        "{}Manager",
        get_composition_name(&composition_category, false)
    );
    scope.new_struct(&*manager_struct)
        .vis("pub");
}
