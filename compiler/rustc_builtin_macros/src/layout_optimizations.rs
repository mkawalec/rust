
use rustc_ast::tokenstream::TokenStream;
use rustc_ast::{token, Expr, ExprKind, Ty};
use rustc_expand::base::{self, *};
use rustc_span::Span;

#[instrument]
pub fn expand<'cx>(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'cx> {
    warn!("foobarz2");
    let mut parser = cx.new_parser_from_tts(tts);

    let ty = parser.parse_ty().unwrap();
    parser.expect(&token.Comma).unwrap();
    let expr = parser.parse_expr().unwrap();

    let layout_expr = cx.expr(span(ExprKind::Type()))
    //compile_error!("We executed the macro!".concat(format!("{:?}, {:?}", meta_item, item)));

    // Looks like we can go ast::Item -> kind -> Struct::VariantData (0) -> FieldDef
    // to access the type on both Structs and Tuples and set the TyKind
    DummyResult::any(span)
}
