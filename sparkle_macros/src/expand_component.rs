use syntax::parse::token;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast::{MetaItem, Item};
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::{TraitDef, MethodDef, combine_substructure};
use syntax::ext::deriving::generic::ty::{Path, LifetimeBounds, Literal};
use syntax::ext::base::{ItemDecorator, ExtCtxt};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct ComponentDecorator {
    pub index_counter: AtomicUsize
}

impl ComponentDecorator {
    pub fn new() -> ComponentDecorator {
        ComponentDecorator {
            index_counter: AtomicUsize::new(0)
        }
    }
}

impl ItemDecorator for ComponentDecorator {
    fn expand(&self, cx: &mut ExtCtxt,
                     span: Span,
                     mitem: &MetaItem,
                     item: &Item,
                     mut push: Box<FnMut(P<Item>)>)
    {
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
                    name: "index_of",
                    generics: LifetimeBounds::empty(),
                    explicit_self: None,
                    args: Vec::new(),
                    ret_ty: Literal(Path::new(vec!("usize"))),
                    attributes: attrs,
                    combine_substructure: combine_substructure(box |&: c, s, _sub| {
                        c.expr_uint(s, self.index_counter.fetch_add(1, Ordering::SeqCst))
                    })
                }
            )
        };

        component_trait_def.expand(cx, mitem, item, |p| push.call_mut((p,)));
    }
}
