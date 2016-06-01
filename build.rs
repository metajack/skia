/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cmake;

use std::env;

fn main() {
    let dst = cmake::Config::new(".").generator("ninja").build_target("all").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=stdc++");
//    println!("cargo:outdir={}", out_dir);

    let target = env::var("TARGET").unwrap();
    if target.contains("unknown-linux-gnu") {
        println!("cargo:rustc-link-lib=bz2");
        println!("cargo:rustc-link-lib=GL");
    } else if target.contains("eabi") {
        println!("cargo:rustc-link-lib=GLESv2");
    } else if target.contains("apple-darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=usp10");
        println!("cargo:rustc-link-lib=ole32");
    }
}
