use crate::util::check_builtin_macro_attribute;

use rustc_ast::{self as ast};
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::Span;
use rustc_span::symbol::{sym};
use std::ops::Deref;

#[instrument]
pub fn expand(
    ecx: &mut ExtCtxt<'_>,
    _span: Span,
    meta_item: &ast::MetaItem,
    item: Annotatable,
) -> Vec<Annotatable> {
    check_builtin_macro_attribute(ecx, meta_item, sym::layout_optimizations);
    warn!("foobarz2");
    //compile_error!("We executed the macro!".concat(format!("{:?}, {:?}", meta_item, item)));

    // Looks like we can go ast::Item -> kind -> Struct::VariantData (0) -> FieldDef
    // to access the type on both Structs and Tuples and set the TyKind
    match item {
        Annotatable::Item(ref item) => {
            match item.deref() {
                ast::Item { kind: ast::ItemKind::Struct(variant_data, _), ..} => {
                    info!("first foobar: {:?}", variant_data);
                },
                _ => {
                    info!("second foobar");
                }
            }
        },
        _ => info!("third foobar")
    };
    vec![item]
}
