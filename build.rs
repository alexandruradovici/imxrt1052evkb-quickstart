use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use cc::Build;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    // assemble the `asm.s` file
    Build::new().file("hdr.s").compile("asm"); // <- NEW!

    // rebuild if `asm.s` changed
    println!("cargo:rerun-if-changed=hdr.s"); // <- NEW!

}
