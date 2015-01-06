#![crate_name = "sparkle_macros"]
#![unstable]
#![feature(
    plugin_registrar,
    macro_rules,
    unboxed_closures,
    slicing_syntax,
    quote
)]

extern crate rustc;
extern crate syntax;

use rustc::plugin;
use syntax::parse::token;
use syntax::ext::base::SyntaxExtension;

mod expand_component;
mod expand_get_stores;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("SparkleComponent"),
        SyntaxExtension::Decorator(box expand_component::ComponentDecorator::new())
    );

    reg.register_macro("sparkle_get_stores", expand_get_stores::expand)
}

#[macro_export]
macro_rules! sparkle_entity(
    ($em:expr, [$($component:expr),+]) => ({
        let entity = $em.create();
        $(
            $em.attach_component(&entity, $component);
        )+

        entity
    })
);

#[macro_export]
macro_rules! sparkle_filter(
    (require components: $($mandatory_component_type:ident),*
     forbid components: $($forbidden_component_type:ident),*
     require groups: $($mandatory_group:expr),*
     forbid groups: $($forbidden_group:expr),*
     ) => ({
        let mut filter = sparkle::system::Filter::new();
        $(
            filter.require_component::<$mandatory_component_type>();
        )*
        $(
            filter.forbid_component::<$forbidden_component_type>();
        )*
        $(
            filter.require_group($mandatory_group);
        )*
        $(
            filter.forbid_group($forbidden_group);
        )*
        filter
    })
);
