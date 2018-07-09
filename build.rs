use std::env;

#[cfg(windows)] const OPENNI2_DIR64: &str = "OPENNI2_LIB64";
#[cfg(windows)] const OPENNI2_DIR: &str = "OPENNI2_LIB";
#[cfg(not(windows))] const OPENNI2_DIR64: &str = "OPENNI2_REDIST64";
#[cfg(not(windows))] const OPENNI2_DIR: &str = "OPENNI2_REDIST";

fn main() {
    let openni2_dir = env::var(OPENNI2_DIR)
                      .or(env::var(OPENNI2_DIR64))
                      .expect(&format!("Required env var for dynamic libraries missing. Expected `{}` or `{}`.", OPENNI2_DIR, OPENNI2_DIR64));
    println!("cargo:rustc-link-search=native={}", openni2_dir);
    println!("cargo:rustc-link-lib=dylib=openni2");
}
