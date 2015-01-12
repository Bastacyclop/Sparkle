use std::collections::HashSet;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::ast::{TokenTree, Expr, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacExpr};
use syntax::ext::build::AstBuilder;

#[doc(hidden)]
pub fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {

    let (store_map, component_idents) = match parse_args(cx, sp, tts) {
        Some(result) => result,
        None => return DummyResult::any(sp)
    };

    let mut ensure_stmts = Vec::new();
    for component_ident in component_idents.iter() {
        ensure_stmts.push(quote_stmt!(cx,
            $store_map.ensure::<$component_ident>();
        ));
    }

    let mut tuple_exprs = Vec::new();
    for component_ident in component_idents.iter() {
        tuple_exprs.push(quote_expr!(cx,
            $store_map.get_mut::<$component_ident>().unwrap()
        ));
    }
    let tuple_expr = cx.expr_tuple(sp, tuple_exprs);

    let result_block = cx.block(sp, ensure_stmts, Some(tuple_expr));
    let result_expr = cx.expr_block(result_block);

    return MacExpr::new(result_expr);
}

fn parse_args(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Option<(P<Expr>, Vec<Ident>)> {
    let mut parser = cx.new_parser_from_tts(tts);

    if parser.token == token::Eof {
        cx.span_err(sp, "requires at least the SpaceProxy and one component ident");
        return None
    }

    let store_map = parser.parse_expr();

    let mut component_idents = Vec::new();
    let mut names: HashSet<String> = HashSet::new();
    while parser.token != token::Eof {
        if !parser.eat(&token::Comma) {
            cx.span_err(sp, "expected token `,`");
            return None
        }

        if parser.token.is_ident() {
            let ident = parser.parse_ident();

            let interned_name = token::get_ident(ident);
            let name = interned_name.get();
            if names.contains(name) {
                cx.span_err(sp, format!("duplicate component ident: {}", name).as_slice());
                return None
            }

            component_idents.push(ident);
            names.insert(name.to_string());
        } else {
            cx.span_err(sp, "expected component ident.");
            return None;
        }
    }

    Some((store_map, component_idents))
}