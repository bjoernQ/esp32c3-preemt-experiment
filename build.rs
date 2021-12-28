use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

use riscv_target::Target;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("esp32c3-memory.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    let name = env::var("CARGO_PKG_NAME").unwrap();
    let target = env::var("TARGET").unwrap();

    if target.starts_with("riscv") {
        let mut target = Target::from_target_str(&target);
        target.retain_extensions("if");

        let target = target.to_string();

        fs::copy(
            format!("bin/trap_{}.a", target),
            out.join(format!("lib{}.a", name)),
        )
        .unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out.display());
    }
}
