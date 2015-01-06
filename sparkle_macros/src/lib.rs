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
    ($($component_type:ident),*) => ({
        let mut filter = sparkle::system::Filter::new();
        $(
            filter.insert_mandatory::<$component_type>();
        )*

        filter
    })
);