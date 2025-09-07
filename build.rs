use std::env;

fn main() {
    // Only add Windows icon resource if we're building for Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // If we have a .rc file or .ico file, we can add it here
        println!("cargo:rustc-link-arg=/SUBSYSTEM:CONSOLE");
    }
    
    // Embed assets at compile time
    println!("cargo:rerun-if-changed=assets/logo.png");
    println!("cargo:rerun-if-changed=assets/banner.png");
}