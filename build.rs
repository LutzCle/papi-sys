/*
 * Copyright 2018 German Research Center for Artificial Intelligence (DFKI)
 * Author: Clemens Lutz <clemens.lutz@dfki.de>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let link_option = match env::var("PAPI_LIBRARY") {
        Ok(val) => {
            println!("cargo:rustc-link-lib=static=papi");
            println!("cargo:rustc-link-search=native={}", val);

            format!("-L{}", val)
        }
        Err(_) => "-Lpapi".to_string(),
    };

    let include_option = match env::var("PAPI_INCLUDE_DIR") {
        Ok(val) => format!("-I{}", val),
        Err(_) => "-Ipapi".to_string(),
    };

    let bindings = match env::var("PAPI_INCLUDE_DIR") {
        Ok(val) => bindgen::builder().clang_arg(format!("-I{}", val)),
        Err(_) => bindgen::builder(),
    };

    let bindings = bindings
        .header("papi_wrapper.c")
        .clang_arg(&link_option)
        .clang_arg(&include_option)
        .whitelist_recursively(false)
        .whitelist_type("^PAPI_[[:alpha:]_]+")
        .whitelist_type("^_papi_[[:alpha:]_]+")
        .whitelist_function("^PAPI_[[:alpha:]_]+")
        .whitelist_function("^_papi_[[:alpha:]_]+")
        .whitelist_var("^PAPI_[[:alpha:]_]+")
        .whitelist_var("^_papi_[[:alpha:]_]+")
        .whitelist_type("caddr_t")
        .whitelist_type("__caddr_t")
        .whitelist_type("_dmem_t")
        .whitelist_type("event_info")
        .generate()
        .expect("Unable to generate PAPI bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write PAPI bindings");

    cc::Build::new()
        .file("papi_wrapper.c")
        .static_flag(true)
        .flag(&link_option)
        .flag(&include_option)
        .compile("libpapi_sys.a");
}
