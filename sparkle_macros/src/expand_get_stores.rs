use std::collections::HashSet;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::ast::{TokenTree, Expr, Ident};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacExpr};
use syntax::ext::build::AstBuilder;

#[doc(hidden)]
pub fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {

    let (cm, mut component_idents) = match parse_args(cx, sp, tts) {
        Some(result) => result,
        None => return DummyResult::any(sp)
    };

    let mut stmts = Vec::new();
    for component_ident in component_idents.iter() {
        stmts.push(quote_stmt!(cx,
            $cm.ensure::<$component_ident>();
        ));
    }
    
    stmts.push(quote_stmt!(cx, 
        let raw_mapper: *mut ComponentMapper = &mut *$cm;
    ));

    let result_expr;
    if component_idents.len() > 1 {
        let mut tuple_exprs = Vec::new();
        for component_ident in component_idents.iter() {
            tuple_exprs.push(quote_expr!(cx,
                unsafe {
                    (*raw_mapper).get_store_mut::<$component_ident>()
                }
            ));
        }

        let tuple_expr = cx.expr_tuple(sp, tuple_exprs);
        let result_block = cx.block(sp, stmts, Some(tuple_expr));
        result_expr = cx.expr_block(result_block);
    } else {
        let component_ident = component_idents.pop();
        let get_store_expr = quote_expr!(cx, 
            unsafe {
                (*raw_mapper).get_store_mut::<$component_ident>()
            }
        );

        let result_block = cx.block(sp, stmts, Some(get_store_expr));
        result_expr = cx.expr_block(result_block);
    }

    return MacExpr::new(result_expr);
}

fn parse_args(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Option<(P<Expr>, Vec<Ident>)> {
    let mut parser = cx.new_parser_from_tts(tts);

    if parser.token == token::Eof {
        cx.span_err(sp, "requires at least the SpaceProxy and one component ident");
        return None
    }

    let cm = parser.parse_expr();

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

    Some((cm, component_idents))
}
