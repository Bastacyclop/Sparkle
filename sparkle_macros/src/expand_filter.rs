use std::collections::HashMap;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacExpr};
use syntax::ext::build::AstBuilder;

#[doc(hidden)]
pub fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
    let mut parser = cx.new_parser_from_tts(tts);

    if parser.token == token::Eof {
        let empty_filter = quote_expr!(cx, sparkle::system::filter::Filter::new());
        return MacExpr::new(empty_filter);
    }

    let mut components = HashMap::new();
    components.insert("require", Vec::new());
    components.insert("forbid", Vec::new());
    let mut groups = HashMap::new();
    groups.insert("require", Vec::new());
    groups.insert("forbid", Vec::new());

    while parser.token != token::Eof {
        if !parser.token.is_ident() {
            cx.span_err(sp, "expected ident `require` of `forbid`");
            return DummyResult::any(sp);
        }

        let filter_type_ident = parser.parse_ident();
        let interned_filter_type = token::get_ident(filter_type_ident);
        let filter_type = interned_filter_type.get();

        match filter_type {
            "require" => {}
            "forbid" => {}
            _ => {
                cx.span_err(sp, "expected ident `require` of `forbid`");
                return DummyResult::any(sp);
            }
        }

        if !parser.token.is_ident() {
            cx.span_err(sp, "expected ident `components` or `groups`");
            return DummyResult::any(sp);
        }

        let filter_category_indent = parser.parse_ident();
        let interned_filter_category = token::get_ident(filter_category_indent);
        let filter_category = interned_filter_category.get();

        match filter_category {
            "components" => {
                if !parser.eat(&token::Colon) {
                    cx.span_err(sp, "expected token `:`");
                    return DummyResult::any(sp);
                }

                loop {
                    if !parser.token.is_ident() {
                        cx.span_err(sp, "expected component type");
                        return DummyResult::any(sp);
                    }

                    let component_type = parser.parse_ident();
                    components.get_mut(filter_type).map(|v| v.push(component_type));

                    if !parser.eat(&token::Comma) {
                        break;
                    }
                }
            }
            "groups" => {
                if !parser.eat(&token::Colon) {
                    cx.span_err(sp, "expected token `:`");
                    return DummyResult::any(sp);
                }

                loop {
                    if !parser.token.can_begin_expr() {
                        cx.span_err(sp, "expected group string");
                        return DummyResult::any(sp);
                    }

                    let group = parser.parse_expr();
                    groups.get_mut(filter_type).map(move |v| v.push(group));

                    if !parser.eat(&token::Comma) {
                        break;
                    }
                }
            }
            _ => {
                cx.span_err(sp, "expected ident `components` or `groups`");
                return DummyResult::any(sp);
            }
        }
    }

    let mut creation_stmts = Vec::new();
    creation_stmts.push(
        quote_stmt!(cx, let mut filter = ::sparkle::system::filter::StandardEntityFilter::new();)
    );
    components.get_mut("require").map(|v| {
        for mandatory_component_type in v.iter() {
            creation_stmts.push(
                quote_stmt!(cx, filter.require_component::<$mandatory_component_type>())
            );
        }
    });
    components.get_mut("forbid").map(|v| {
        for forbidden_component_type in v.iter() {
            creation_stmts.push(
                quote_stmt!(cx, filter.forbid_component::<$forbidden_component_type>())
            );
        }
    });
    groups.get_mut("require").map(|v| {
        for mandatory_group in v.iter() {
            creation_stmts.push(
                quote_stmt!(cx, filter.require_group($mandatory_group))
            );
        }
    });
    groups.get_mut("forbid").map(|v| {
        for forbidden_group in v.iter() {
            creation_stmts.push(
                quote_stmt!(cx, filter.forbid_group($forbidden_group);)
            );
        }
    });

    let creation_block = cx.block(sp, creation_stmts, Some(quote_expr!(cx, filter)));
    let result_expr = cx.expr_block(creation_block);
    return MacExpr::new(result_expr);
}
