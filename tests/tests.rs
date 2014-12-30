#![feature(phase)]

#[phase(plugin, link)]
extern crate sparkle;

mod test_entity_pool;
mod test_entity_updates_record;
mod test_component_deriving;
mod test_component_store;
mod test_system_filter;