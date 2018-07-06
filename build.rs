use std::env;

fn main() {
    let openni2_dir = env::var("OPENNI2_REDIST").expect("Required env var missing: OPENNI2_REDIST");
    println!("cargo:rustc-link-search=native={}", openni2_dir);
    println!("cargo:rustc-link-lib=dylib=openni2");
}
