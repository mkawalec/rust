use rustc_ast::ptr::P;
use rustc_ast::tokenstream::TokenStream;
use rustc_ast::{token, TyKind, Ty, Pat, PatKind};
use rustc_errors::PResult;
use rustc_expand::base::{self, *};
use rustc_parse::parser::{RecoverColon, RecoverComma, CommaRecoveryMode};
use rustc_span::Span;

#[instrument]
pub fn expand<'cx>(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    tts: TokenStream,
) -> Box<dyn base::MacResult + 'cx> {
    let (ty, pat) = match parse_pattern_type(cx, span, tts) {
        Ok(parsed) => parsed,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(span);
        }
    };

    let pattern_ty = cx.ty(span, TyKind::Pat(ty, pat));
    return MacEager::ty(pattern_ty);
}

fn parse_pattern_type<'a>(cx: &mut ExtCtxt<'a>, span: Span, tts: TokenStream) -> PResult<'a, (P<Ty>, P<Pat>)> {
    let mut parser = cx.new_parser_from_tts(tts);

    let ty = parser.parse_ty()?;
    parser.expect(&token::Comma)?;
    let pattern = parser.parse_pat_allow_top_alt(None, RecoverComma::No, RecoverColon::No, CommaRecoveryMode::LikelyTuple).unwrap();

    let PatKind::Range(..) = pattern.kind else {
        let mut err = cx.struct_span_err(span, "Pattern types only support range expressions");
        err.span_suggestion(
            parser.token.span,
            "try supplying a range expression instead",
            "",
            rustc_errors::Applicability::MaybeIncorrect
        );
        return Err(err);
    };

    Ok((ty, pattern))
}
