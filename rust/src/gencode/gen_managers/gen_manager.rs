use codegen::{Block, Function, Scope};

use crate::{
    compositions::CompositionCategory,
    gencode::gen_managers::{builder::ArmsBlock, helpers::get_composition_paths, composition_type_manager_method::CompositionTypeManagerMethod},
};
use crate::gencode::gen_managers::builder::ArgFunction;
use crate::gencode::gen_managers::helpers::get_all_composition_res_variants_enums::get_composition_response_enum;
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;

pub fn gen_manager(scope: &mut Scope, composition_category: &CompositionCategory) {
    let composition_type = get_composition_name(&composition_category, true);
    let composition_name = get_composition_name(&composition_category, false);
    let (_, create_request) = get_composition_paths::get_composition_create_request_path(&composition_category);
    let composition_response = get_composition_response_enum(&composition_category);
    let generics = format!("{composition_type}, {create_request}, {composition_response}");

    let gen_method = get_method(composition_category);

    scope
        .new_impl(format!("{}Manager", composition_name).as_str())
        .impl_trait(format!("CompositionTypeManager<{}>", generics))
        .push_fn(
            gen_method(
                CompositionTypeManagerMethod::GetPublic,
                vec![
                    ("composition_type", &composition_type),
                    ("composition_source_id", "u128"),
                ],
            )
        )
        .push_fn(
            gen_method(
                CompositionTypeManagerMethod::GetPrivate,
                vec![
                    ("composition_type", &composition_type),
                    ("composition_source_id", "u128"),
                    ("author_id", "u128"),
                ],
            )
        )
        .push_fn(
            gen_method(
                CompositionTypeManagerMethod::Create,
                vec![
                    ("composition_type", format!("&{}", &composition_type).as_str()),
                    ("create_request", "Box<dyn Any>"),
                    ("layout_id", "u128"),
                    ("author_id", "u128"),
                ],
            ),
        )
        .push_fn(
            gen_method(
                CompositionTypeManagerMethod::Update,
                vec![
                    ("composition_type", format!("&{}", &composition_type).as_str()),
                    ("composition_update_que", "Vec<UpdateDataOfComposition>"),
                    ("composition_source_id", "u128"),
                    ("author_id", "u128"),
                ],
            )
        )
        .push_fn(
            gen_method(
                CompositionTypeManagerMethod::Delete,
                vec![
                    ("composition_type", format!("&{}", &composition_type).as_str()),
                    ("composition_source_id", "u128"),
                    ("author_id", "u128"),
                ],
            )
        );
}

fn get_method<'a>(composition_category: &'a CompositionCategory) -> Box<dyn Fn(CompositionTypeManagerMethod, Vec<(&str, &str)>) -> Function + 'a> {
    let composition_type = get_composition_name(&composition_category, true);
    let composition_name = get_composition_name(&composition_category, false);
    let (_, create_request) = get_composition_paths::get_composition_create_request_path(&composition_category);
    let composition_response = get_composition_response_enum(&composition_category);

    Box::new(move |method: CompositionTypeManagerMethod, args: Vec<(&str, &str)>| {
        Scope::new()
            .new_fn(&method.get_method_name().as_str())
            .ret(&composition_response)
            .arg_ref_self()
            .add_args(args)
            .push_block(
                Block::new("match composition_type")
                    .add_arms(&method, composition_category, &method)
                    .to_owned(),
            )
            .to_owned()
    })
}
