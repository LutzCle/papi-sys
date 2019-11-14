// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Copyright 2019 Yeonsoo Kim
//
// Authors:
//   Clemens Lutz <clemens.lutz@dfki.de>
//   Yeonsoo Kim <alkorang@outlook.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

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
