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
        token::intern("SparkleComponent"),
        SyntaxExtension::Decorator(Box::new(expand_component::ComponentDecorator::new()))
    );

    reg.register_macro("sparkle_get_stores", expand_get_stores::expand);
    reg.register_macro("sparkle_filter", expand_filter::expand);
}



#[macro_export]
macro_rules! _sparkle_add_entity {
    ($system:expr, $mentity:expr) => ({
        $system.entities.insert($mentity.entity);
        $system.processor.on_entity_added($mentity);
    })
}

#[macro_export]
macro_rules! _sparkle_remove_entity {
    ($system:expr, $mentity:expr) => ({
        $system.entities.remove(&$mentity.entity);
        $system.processor.on_entity_removed($mentity);
    })
}

#[macro_export]
macro_rules! sparkle_default_system_filtering {
    () => (
        fn on_entity_created(&mut self, mentity: &sparkle::MetaEntity) {
            if self.filter.pass(mentity) {
                _sparkle_add_entity!(self, mentity);
            }
        }

        fn on_entity_changed(&mut self, mentity: &sparkle::MetaEntity) {
            let contains = self.entities.contains(&mentity.entity);
            let pass_filter = self.filter.pass(mentity);

            match (contains, pass_filter) {
                (true, false) => _sparkle_remove_entity!(self, mentity),
                (false, true) => _sparkle_add_entity!(self, mentity),
                _ => {}
            }
        }

        fn on_entity_removed(&mut self, mentity: &sparkle::MetaEntity) {
            _sparkle_remove_entity!(self, mentity);
        }
    )
}

