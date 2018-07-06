use std::env;

fn main() {
    let openni2_dir = env::var("OPENNI2_LIB")
                      .or(env::var("OPENNI2_LIB64"))
                      .or(env::var("OPENNI2_REDIST"))
                      .or(env::var("OPENNI2_REDIST64"))
                      .expect("Required env var for dynamic libraries missing. Need one of:\nOPENNI2_LIB64 or OPENNI2_LIB (location of openni2.lib on Windows)\nOPENNI2_REDIST64 or OPENNI2_REDIST (location of libOpenNI2.dylib/libOpenNI2.so on OSX/Linux)");
    println!("cargo:rustc-link-search=native={}", openni2_dir);
    println!("cargo:rustc-link-lib=dylib=openni2");
}
