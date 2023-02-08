# KallistiOS Rust Bindings

This repository contains Rust bindings for the KallistiOS API. This is meant to provide a memory-safe Rust interface to interact
with the Dreamcast hardware. The library is separated into individual modules for each of the different functionality it provides
(e.g. `pvr` for PowerVR structures and functions). All of the raw bindings to the KallistiOS C library should be added to the `c_raw`
module.

Due to GCCRS (the version of Rust used to compile these bindings) being in its infancy, this crate is not able to be consumed in its
current form. In order to provide a usable interface, the `generate.py` Python script is used to combine all of the individual modules
into a monolithic `kos.rs` file that can be included in a project and compiled into the final binary.

After the `generate.py` script has been run, all of the KallistiOS bindings can be used by adding the library as a module in your project:

```rust
mod kos;
```

