#![feature(collections)]
#![feature(unicode)]
#![feature(core)]

extern crate "rustc-serialize" as rustc_serialize;

pub mod base64;
pub mod fixed_xor;
pub mod frequency_analysis;
pub mod single_byte_xor;
pub mod repeating_xor;
pub mod utility;
