#![allow(non_snake_case)]

use std::{
    cell::OnceCell,
    collections::HashMap,
    ffi::{c_void, CStr, CString, OsStr},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use hyoubkp::datagen::{DataGenDispatch, DataGenKind};
use hyoubkp::executor::Executor;
use hyoubkp::tokmap::TokenMapperKind;
use hyoubkp_base::datagen::DataGen;
#[allow(unused_imports)]
use hyoubkp_base::tokmap::TokenMapperOption;

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
extern "C" fn app_action_MainViewController_self_Load(
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

    #[allow(unused_mut)]
    let mut tokmap_options = HashMap::new();

    let tokmap_kind;
    let mut tokmap_version = "";

    #[cfg(feature = "tokmap_user")]
    {
        tokmap_kind = TokenMapperKind::User;
    }

    #[cfg(feature = "tokmap_rule")]
    {
        tokmap_kind = TokenMapperKind::Rule;
        tokmap_options.insert(
            TokenMapperOption::RuleFile,
            rule_file_name.to_string_lossy().into_owned().clone(),
        );
    }

    match Executor::new(tokmap_kind, &tokmap_options) {
        Ok(mut executor) => {
            executor.enable_realtime_date();

            tokmap_version = executor.get_tokmap_version();

            unsafe {
                APPCTX
                    .set(AppMainView {
                        executor,
                        number,
                        output_file_name,
                        output_file_name_backup: document_path.join("output.csv.bak"),
                        rule_file_name,
                    })
                    .unwrap()
            };

            let label1 = unsafe { appui_MainViewController_label1(vc) };
            unsafe { appui_uikit_label_set_text(label1, c"[waiting for input]".as_ptr()) };
        }
        Err(e) => {
            let e = CString::new(e.to_string()).unwrap_or_default();

            extern "C" fn callback(_action: *mut c_void, _userdata: *mut c_void) {
                std::process::exit(1);
            }
            unsafe {
                appui_uikit_alertctrl(
                    vc,
                    c"Error".as_ptr(),
                    e.as_ptr(),
                    callback as *mut c_void,
                    std::ptr::null_mut(),
                )
            };
        }
    }

    let foot_text = CString::new(format!(
        r##"App version: {}, Hyoubkp version: {}, token-mapper: {} {}

Copyright © 2024 Eric Tian <thxdaemon@gmail.com>. All rights reserved.
This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>."##,
        env!("CARGO_PKG_VERSION"),
        hyoubkp::VERSION,
        tokmap_kind.as_str(),
        tokmap_version
    )).unwrap_or_default();

    unsafe {
        let label = appui_MainViewController_footLabel(vc);
        appui_uikit_label_set_text(label, foot_text.as_ptr());
    }
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

    match ctx.executor.parse_expr(expr) {
        Ok(trans) => {
            let output_file_name = &ctx.output_file_name;
            let output_file_name_backup = &ctx.output_file_name_backup;

            let datagen_impl = DataGenDispatch::new(DataGenKind::GnuCash);

            if output_file_name.exists() {
                std::fs::copy(output_file_name, output_file_name_backup).unwrap();
            }

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
        Err(e) => {
            let e = CString::new(e.to_string()).unwrap_or_default();
            unsafe {
                appui_uikit_alertctrl(
                    vc,
                    c"Error".as_ptr(),
                    e.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };
        }
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
    .unwrap_or_default();

    unsafe {
        appui_uikit_label_set_text(label1, text.as_ptr());
    }
}
