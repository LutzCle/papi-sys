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
    // PAPI_LIBRARY
    match env::var("PAPI_LIBRARY") {
        Ok(val) => println!("cargo:rustc-link-search=native={}", val),
        Err(_) => {},
    }
    println!("cargo:rustc-link-lib=static=papi");

    // PAPI_INCLUDE_DIR
    let bindings = match env::var("PAPI_INCLUDE_DIR") {
        Ok(val) => bindgen::builder().clang_arg(format!("-I{}", val)),
        Err(_) => bindgen::builder(),
    };

    let bindings = bindings
        .header("wrapper.h")
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

}
