#![crate_name = "enigma"]
#![crate_type = "lib"]

#![feature(core)]

//#![deny(missing_doc)]

extern crate "rustc-serialize" as rustc_serialize;

#[allow(dead_code)]
pub mod ciphers;
#[allow(dead_code)]
pub mod hashes;
#[allow(dead_code)]
pub mod util;
