#![crate_name = "sparkle"]
#![unstable]
#![feature(slicing_syntax, associated_types)]

extern crate rustc;
extern crate syntax;

pub mod entity;
pub mod component;
pub mod group;
pub mod tag;
pub mod system;
pub mod space;