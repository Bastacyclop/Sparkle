#![crate_name = "sparkle_macros"]
#![unstable]
#![allow(unstable)]
#![feature(
    plugin_registrar,
    unboxed_closures,
    box_syntax,
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
mod expand_filter;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("component"),
        SyntaxExtension::Decorator(Box::new(expand_component::ComponentDecorator::new()))
    );

    reg.register_macro("sparkle_get_stores", expand_get_stores::expand);
    reg.register_macro("sparkle_filter", expand_filter::expand);
}
