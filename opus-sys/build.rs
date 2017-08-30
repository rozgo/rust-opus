extern crate pkg_config;
use std::process::Command;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let lib_path = out_dir.join("x86_64");
    println!("cargo:root={}", out_dir.display());
    println!("lib_path={}", lib_path.display());
    let p = lib_path.to_str().unwrap();
    println!("cargo:rustc-flags=-L native={} -l static=opus2015", p);
}

