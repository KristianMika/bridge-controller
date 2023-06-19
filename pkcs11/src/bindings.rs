#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// TODO: use std::path::MAIN_SEPARATOR_STR when stable
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
