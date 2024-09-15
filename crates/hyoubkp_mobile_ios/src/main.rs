#![no_main]

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod hmui;
pub mod app_main_view;

use crate::hmui::*;

use std::ffi::{c_char, c_int};

#[no_mangle]
extern "C" fn main(argc: c_int, argv: *mut *mut c_char) {
    std::process::exit(unsafe { appui_main(argc, argv) });
}
