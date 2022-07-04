use std::{process::Command, path::Path};

// syscalls

fn main() {
    let out_dir = std::env::var("OUT_DIR")
        .unwrap();

    Command::new("fasm")
        .arg("src/asm/syscalls.asm")
        .arg(&Path::new(&out_dir).join("syscalls.o"))
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crus", "libsyscalls.a", "syscalls.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=syscalls");
    println!("cargo:rerun-if-changed=src/asm/syscalls.asm");
}
