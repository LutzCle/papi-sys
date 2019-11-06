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

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let papi_prefix_path = env::var("PAPI_PREFIX").map(|p| PathBuf::from(p)).ok();

    let clang_args = if let Some(p) = papi_prefix_path {
        println!("cargo:rustc-link-search={}", p.join("lib").display());
        println!("cargo:rust-flags=-L{}", p.join("lib").display());

        vec![
            format!("-I{}", p.join("include").display()),
            format!("-L{}", p.join("lib").display()),
        ]
    } else {
        Vec::new()
    };

    println!("cargo:rustc-link-lib=papi");

    bindgen::builder()
        .rustfmt_bindings(false)
        .header("wrapper.h")
        .clang_args(clang_args.iter())
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
        .expect("Unable to generate PAPI bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write PAPI bindings");

    let codegen_stdout = Command::new("sh")
        .arg("codegen.sh")
        .output()
        .unwrap()
        .stdout;
    let mut file = File::create(out_path.join("codegen.rs"))?;
    file.write_all(&codegen_stdout)?;
    file.sync_all()?;

    Ok(())
}
