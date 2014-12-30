use rustc::plugin;
use syntax::parse::token;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast::{MetaItem, Item};
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::{TraitDef, MethodDef, combine_substructure};
use syntax::ext::deriving::generic::ty::{Path, LifetimeBounds, Self, Literal};
use syntax::ext::base::{ItemDecorator, SyntaxExtension, ExtCtxt};
use std::sync::atomic::{AtomicUint, SeqCst};

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("component"),
        SyntaxExtension::Decorator(box ComponentDecorator { index_counter: AtomicUint::new(0u) })
    )
}

struct ComponentDecorator {
    index_counter: AtomicUint
}

impl ItemDecorator for ComponentDecorator {
    fn expand(&self, cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Item, push: |P<Item>|) {
        let inline = cx.meta_word(span, token::InternedString::new("inline"));
        let attrs = vec!(cx.attribute(span, inline));

        let component_trait_def = TraitDef {
            span: span,
            attributes: Vec::new(),
            path: Path::new(vec!("sparkle", "component", "Component")),
            additional_bounds: Vec::new(),
            generics: LifetimeBounds::empty(),
            methods: vec!(
                MethodDef {
                    name: "get_type_index",
                    generics: LifetimeBounds::empty(),
                    explicit_self: None,
                    args: vec!(
                        Literal(Path::new_(
                            vec!("std", "option", "Option"),
                            None,
                            vec!(box Self),
                            true
                        ))
                    ),
                    ret_ty: Literal(Path::new(vec!("uint"))),
                    attributes: attrs,
                    combine_substructure: combine_substructure(|c, s, _sub|{
                        c.expr_uint(s, self.index_counter.fetch_add(1, SeqCst))
                    })
                }
            )
        };

        component_trait_def.expand(cx, mitem, item, |p| push(p));
    }
}