use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let linker_script = Path::new(&manifest_dir).join("kernel.ld");

    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
    println!("cargo:rustc-link-arg=-no-pie");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rerun-if-changed={}", linker_script.display());

    println!(
        "cargo:warning=Using linker script: {}",
        linker_script.display()
    );
}
