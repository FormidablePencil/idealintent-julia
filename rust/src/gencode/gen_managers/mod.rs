use std::{fs::File, io::Write};

use codegen::Scope;

use crate::{compositions::CompositionCategory, get_composition_name};
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;

use self::gen_manager::gen_manager;
use self::gen_manager_dependencies::gen_manager_dependencies;

mod builder;
mod composition_type_manager_method;
mod gen_manager;
mod gen_manager_dependencies;
mod helpers;
mod import_dependencies;

// todo - Make sure codegen doesn't enter production

pub fn write_to_file(file_name: &str, contents: &mut String) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    // assert_eq!(contents, "Hello, world!");
    Ok(())
}

pub fn impl_composition_type_manager(composition_category: CompositionCategory) {
    let composition_name = get_composition_name!(&composition_category).to_ascii_lowercase();
    let model_name = format!("{composition_name}s");

    let mut scope = Scope::new();
    scope.raw("\
// ==============================================================================================
//  Please don't edit this file directly. This file is auto generated.
// ==============================================================================================");

    gen_manager_dependencies(&mut scope, &composition_category);

    gen_manager(&mut scope, &composition_category);

    match write_to_file(
        format!("src/compositions/{}/manager.rs", model_name).as_str(),
        &mut scope.to_string(),
    ) {
        Ok(_) => print!("success"),
        Err(_) => todo!("failed"),
    }

    println!("{}", scope.to_string());
}

#[cfg(test)]
mod carousel {
    use crate::compositions::banners::banner_enums::BannerType;
    use crate::compositions::carousels::carousel_enums::CarouselType;
    use crate::compositions::CompositionCategory;
    use crate::compositions::paragraphs::paragraph_enums::ParagraphType;

    use super::impl_composition_type_manager;

    #[test]
    fn construct_manager() {
        impl_composition_type_manager(
            // CompositionCategory::Banner(BannerType::Basic),
            // CompositionCategory::Carousel(CarouselType::Basic),
            CompositionCategory::Paragraph(ParagraphType::Basic),
        );
    }
}
