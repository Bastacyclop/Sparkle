use std::collections::HashSet;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::ast::{TokenTree, Expr, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacExpr};
use syntax::ext::build::AstBuilder;

#[doc(hidden)]
pub fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {

    let (em, mut component_idents) = match parse_args(cx, sp, tts) {
        Some(result) => result,
        None => return DummyResult::any(sp)
    };

    let mut tuple_exprs = Vec::new();
    let last_ident = component_idents.pop();
    let last_expr = quote_expr!(cx, ($em).get_store_mut::<$last_ident>());

    for component_ident in component_idents.iter() {
        let expr = quote_expr!(cx,  unsafe {
            let mut_copy: &mut sparkle::entity::Manager = std::mem::transmute(($em));
            mut_copy.get_store_mut::<$component_ident>()
        });

        tuple_exprs.push(expr);
    }

    tuple_exprs.push(last_expr);
    return MacExpr::new(cx.expr_tuple(sp, tuple_exprs));
}

fn parse_args(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Option<(P<Expr>, Vec<Ident>)> {
    let mut parser = cx.new_parser_from_tts(tts);

    if parser.token == token::Eof {
        cx.span_err(sp, "requires at least the SpaceProxy and one component ident");
        return None
    }

    let em_expr = parser.parse_expr();
    let em = cx.expr_mut_addr_of(sp, cx.expr_deref(sp, em_expr));

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

    Some((em, component_idents))
}