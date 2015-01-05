#![crate_name = "sparkle"]
#![feature(
    plugin_registrar, 
    macro_rules, 
    unboxed_closures, 
    slicing_syntax
)]
#![unstable]

extern crate rustc;
extern crate syntax;

pub mod entity;
pub mod component;
pub mod group;
pub mod tag;
pub mod system;
pub mod space;

pub mod macros;