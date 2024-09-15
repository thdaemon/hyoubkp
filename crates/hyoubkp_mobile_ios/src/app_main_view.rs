#![allow(non_snake_case)]

use std::{
    cell::OnceCell,
    ffi::{CStr, CString, OsStr},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use hyoubkp::datagen::{DataGenDispatch, DataGenKind};
use hyoubkp_base::datagen::DataGen;

pub use hyoubkp::executor::Executor;
pub use hyoubkp::tokmap::TokenMapperKind;
pub use hyoubkp_base::transaction::Transaction;

use crate::hmui::*;

#[derive(Debug)]
struct AppMainView {
    executor: Executor,
    number: i32,
    output_file_name: PathBuf,
    output_file_name_backup: PathBuf,
    #[allow(dead_code)]
    rule_file_name: PathBuf,
}

static mut APPCTX: OnceCell<AppMainView> = OnceCell::new();

static USERDEFAULTS_KEY_NUMBER: &CStr = c"hm_number";

#[no_mangle]
unsafe extern "C" fn app_action_MainViewController_self_Load(
    vc: *mut ::std::os::raw::c_void,
    _sender: *mut ::std::os::raw::c_void,
    _event: *mut ::std::os::raw::c_void,
) {
    let document_path = unsafe { CStr::from_ptr(appui_fs_document_path()) };
    let document_path = Path::new(OsStr::from_bytes(document_path.to_bytes()));

    let output_file_name = document_path.join("output.csv");
    let rule_file_name = document_path.join("rule.toml");

    if !std::fs::exists(&rule_file_name).unwrap() {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&rule_file_name)
            .unwrap();
    }

    let mut number = unsafe { appui_userdefaults_get_i32(USERDEFAULTS_KEY_NUMBER.as_ptr()) };

    if number < 0 {
        number = 0;
    }

    // If the output file is moved out by user, reset number to zero, then the GnuCash backend will add title row
    if !std::fs::exists(&output_file_name).unwrap()
        || std::fs::metadata(&output_file_name).unwrap().len() == 0
    {
        number = 0;
    } else {
        if number == 0 {}
    }

    APPCTX
        .set(AppMainView {
            executor: Executor::new(TokenMapperKind::User),
            number,
            output_file_name,
            output_file_name_backup: document_path.join("output.csv.bak"),
            rule_file_name,
        })
        .unwrap();

    let label1 = appui_MainViewController_label1(vc);
    appui_uikit_label_set_text(label1, c"[waiting for input]".as_ptr());
}

#[no_mangle]
extern "C" fn app_action_MainViewController_button1_Tapped(
    vc: *mut ::std::os::raw::c_void,
    _sender: *mut ::std::os::raw::c_void,
    _event: *mut ::std::os::raw::c_void,
) {
    let text_field1 = unsafe { appui_MainViewController_textField1(vc) };
    let expr = unsafe { CStr::from_ptr(appui_uikit_textField_get_text(text_field1)) }
        .to_str()
        .unwrap();

    let ctx = unsafe { APPCTX.get_mut().unwrap() };

    let trans = ctx.executor.parse_expr(expr).unwrap_or_default();

    let output_file_name = &ctx.output_file_name;

    let datagen_impl = DataGenDispatch::new(DataGenKind::GnuCash);

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(output_file_name)
        .unwrap();

    datagen_impl
        .write_to(&mut file, std::slice::from_ref(&trans), ctx.number as u32)
        .unwrap();

    ctx.number += 1;
    unsafe {
        appui_userdefaults_set_i32(USERDEFAULTS_KEY_NUMBER.as_ptr(), ctx.number);
    }

    unsafe {
        appui_uikit_textField_set_text(text_field1, c"".as_ptr());
    }
}

#[no_mangle]
extern "C" fn app_action_MainViewController_textField1_DidChange(
    vc: *mut ::std::os::raw::c_void,
    sender: *mut ::std::os::raw::c_void,
    _event: *mut ::std::os::raw::c_void,
) {
    let expr = unsafe { CStr::from_ptr(appui_uikit_textField_get_text(sender)) }
        .to_str()
        .unwrap();

    let executor = unsafe { &mut APPCTX.get_mut().unwrap().executor };

    let label1 = unsafe { appui_MainViewController_label1(vc) };

    let text = CString::new(match executor.parse_expr(expr) {
        Ok(trans) => trans.to_string(),
        Err(e) => e.to_string(),
    })
    .unwrap();

    unsafe {
        appui_uikit_label_set_text(label1, text.as_ptr());
    }
}
