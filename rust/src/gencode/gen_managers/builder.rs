use codegen::{Block, Function};
use strum::IntoEnumIterator;

use crate::{
    compositions::{carousels::carousel_enums::CarouselType, CompositionCategory},
};
use crate::compositions::banners::banner_enums::BannerType;
use crate::compositions::texts::text_enums::TextType;
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;
use crate::gencode::gen_managers::helpers::get_composition_paths;
use crate::gencode::gen_managers::helpers::get_mod::get_mod;

use super::composition_type_manager_method::CompositionTypeManagerMethod;

pub trait ArgFunction {
    fn add_args(
        &mut self,
        data: Vec<(&str, &str)>,
    ) -> &mut Self;
}

impl ArgFunction for Function {
    fn add_args(
        &mut self,
        data: Vec<(&str, &str)>,
        // name: &str,
    ) -> &mut Self {
        for (name, arg) in data.into_iter() {
            self.arg(name, arg);
        };
        self
    }
}

pub trait ArmsBlock {
    fn add_arms(
        &mut self,
        crud_operation: &CompositionTypeManagerMethod,
        composition_category: &CompositionCategory,
        method_type: &CompositionTypeManagerMethod,
    ) -> &mut Self;

    fn add_arm(
        &mut self,
        crud_operation: &CompositionTypeManagerMethod,
        enum_type_name: String,
        composition_category: CompositionCategory,
        method_type: &CompositionTypeManagerMethod,
    ) -> &mut Self;

}

impl ArmsBlock for Block {
    fn add_arms(
        &mut self,
        crud_operation: &CompositionTypeManagerMethod,
        composition_category: &CompositionCategory,
        method_type: &CompositionTypeManagerMethod,
    ) -> &mut Self {
        match composition_category {
            CompositionCategory::Carousel(_) => {
                for item in CarouselType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Carousel(item),
                        &method_type,
                    );
                }
            }
            CompositionCategory::Banner(_) => {
                for item in BannerType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Banner(item),
                        &method_type,
                    );
                }
            }
            CompositionCategory::Text(_) => {
                for item in TextType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Text(item),
                        &method_type,
                    );
                }
            }
        }

        self
    }

    fn add_arm(
        &mut self,
        crud_operation: &CompositionTypeManagerMethod,
        enum_type_name: String,
        composition_category: CompositionCategory,
        method_type: &CompositionTypeManagerMethod,
    ) -> &mut Self {
        // region Assemble arms
        let method_name = crud_operation.get_method_name();
        let args_for_method = crud_operation.get_args_for_method();

        let arm_left = format!(
            "{}::{enum_type_name}",
            get_composition_name(&composition_category, true),
        );

        let arm_right = match method_type {
            CompositionTypeManagerMethod::GetPublic |
            CompositionTypeManagerMethod::GetPrivate |
            CompositionTypeManagerMethod::Update |
            CompositionTypeManagerMethod::Delete => format!(
                "{}Response::{enum_type_name}({}::{method_name}{args_for_method}),",
                get_composition_name(&composition_category, false),
                get_mod(&composition_category),
            ),
            CompositionTypeManagerMethod::Create => format!(
                "Some(req) => {}Response::{enum_type_name}({}::create(req, layout_id, author_id)),",
                get_composition_name(&composition_category, false),
                get_mod(&composition_category)
            )
        };
        // endregion

        // region Push both sides of the arm to code
        match method_type {
            CompositionTypeManagerMethod::GetPublic |
            CompositionTypeManagerMethod::GetPrivate |
            CompositionTypeManagerMethod::Update |
            CompositionTypeManagerMethod::Delete => {
                self.line(format!("{arm_left} => {arm_right}"));
            }
            CompositionTypeManagerMethod::Create => {
                let (_, create_request) = get_composition_paths::get_composition_create_request_path(&composition_category);
                let comp_type_name = get_composition_name(&composition_category, true);
                let matcher = format!("{comp_type_name}::{enum_type_name} => match create_request.downcast_ref::<{create_request}>()");

                let block = Block::new(matcher.as_str())
                    .line(format!(
                        "Some(req) => {}Response::{enum_type_name}({}::create(req, layout_id, author_id)),",
                        get_composition_name(&composition_category, false),
                        get_mod(&composition_category)
                    ))
                    .line("None => panic!(\"failed...\")")
                    .to_owned();

                self.push_block(block.to_owned());
            }
        };
        // endregion

        self
    }
}
