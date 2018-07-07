# openni2-sys: OpenNI2 Bindings

Rust bindings for [OpenNI2](https://github.com/occipital/OpenNI2).

# Compilation

OpenNI2 usually expects to be dynamically linked, and requires env variables
to indicate where the libraries are.

OSX/Linux wants an env var to indicate the location of `libOpenNI2.dylib`/`libOpenNI2.so` (the `OPENNI2_REDIST` env var, per OpenNI2's installation instructions), while Windows wants to know the directory containing `OpenNI2.lib` (the `OPENNI2_LIB` or `OPENNI2_LIB64` env vars, per the same instructions).

# Runtime considerations

Depending on your installation of OpenNI2, executables that use it *might*
correctly locate the libraries in their install location (`OPENNI2_REDIST`) but
much more likely will require them to be in the same directory as the executable.

This will make running tests/executables with Cargo a lot more annoying, as you'll
have to copy the libraries into the correct `target` directory. Blah.

# LICENSE

These bindings are distributed under the MIT license, which I don't exactly
know what it means, but was recommended and idgaf.

This repository contains headers for OpenNI2 with a Primsense Ltd. copyright,
but they exist for reference and are not part of the compiled crate that's
distributed through Crates.io.
