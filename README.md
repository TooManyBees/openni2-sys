# openni2-sys: OpenNI2 Bindings

Rust bindings for [OpenNI2](https://github.com/occipital/OpenNI2).

# Compilation

OpenNI2 usually expects to be dynamically linked, and requires env variables
to indicate where the libraries are.

The build script first checks the env vars `OPENNI2_LIB`/`OPENNI2_LIB64`
which (per the OpenNI2 installation instructions) is the location of `OpenNI2.lib`
on Windows. Then it checks `OPENNI2_REDIST`/`OPENNI2_REDIST64` which is the location
of `libOpenNI2.dylib` or `libOpenNI2.so` on OSX or Linux.

The env var `OPENNI2_LIB(64)` only needs to be set on Windows, so there should not
be any conflicts from the build script checking all four possible variables.

# Runtime considerations

The location in `OPENNI2_REDIST(64)` should be added to your `PATH`, or else
copy `OpenNI2.dll`, `libOpenNI2.dylib`, or `libOpenNI2.so` to the executable's
directory.

# LICENSE

These bindings are distributed under the MIT license, which I don't exactly
know what it means, but was recommended and idgaf.

This repository contains headers for OpenNI2 with a Primsense Ltd. copyright,
but they exist for reference and are not part of the compiled crate that's
distributed through Crates.io.
