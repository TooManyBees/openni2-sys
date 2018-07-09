# openni2-sys: OpenNI2 Bindings

Rust bindings for [OpenNI2](https://github.com/occipital/OpenNI2).

The reference vendor files were built with bindgen using this command, with a few additional edits:

`bindgen vendor/OniCAPI.h -o src/lib.rs --whitelist-function oni.* --whitelist-type Oni.* --whitelist-var ONI_.*`

# Compilation

OpenNI2 usually expects to be dynamically linked, and requires env variables
to indicate where the libraries are.

When building on Windows, the build script checks the presence of the env vars
`OPENNI2_LIB` and `OPENNI2_LIB64` (per the OpenNI2 installation instructions) is the location of `OpenNI2.lib` on Windows. On other platforms, it checks
`OPENNI2_REDIST` and `OPENNI2_REDIST64`, which should be the location of
`libOpenNI2.dylib` or `libOpenNI2.so` on OSX or Linux.

(A Windows OpenNI2 installation should also have the `OPENNI2_REDIST(64)` env
var set, but it's not the location needed to correctly link.)

# Runtime considerations

Different compilations of OpenNI2 require its runtime libraries to be in
different places. Usually, adding `OPENNI2_REDIST(64)` to your `PATH` is
enough. Otherwise, copy `OpenNI2.dll`, `libOpenNI2.dylib`, or `libOpenNI2.so` to the executable's directory.

# LICENSE

These bindings are distributed under the MIT license, which I don't exactly
know what it means, but was recommended and idgaf.

This repository contains headers for OpenNI2 with a Primsense Ltd. copyright,
but they exist for reference and are not part of the compiled crate that's
distributed through Crates.io.
