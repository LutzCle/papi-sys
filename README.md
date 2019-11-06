papi-sys
========

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
papi-sys = "0.3.0"
```

Next, add this to your crate root:

```rust
extern crate papi-sys;
```

Before building, ensure that PAPI is installed on your system.

## What is papi-sys?

The purpose of this crate is to provide 1:1 bindings for papi.h.
PAPI is a library that provides a consistent interface to hardware performance
counters. Visit the [PAPI website](http://icl.utk.edu/papi) for more information.

Note that this crate does not provide a high-level interface to PAPI.

## Environment Variables

There are two environment variables to specify a custom PAPI library path:
- `PAPI_PREFIX`: required to generate `bindings.rs`
- `LD_LIBRARY_PATH`: required to dynamically link `libpapi.so`

Let's assume you installed PAPI in `/opt/papi/5.7.0/`, then you can test by
```bash
$ PAPI_PREFIX=/opt/papi/5.7.0/ LD_LIBRARY_PATH=/opt/papi/5.7.0/lib:$LD_LIBRARY_PATH cargo test
```

To avoid setting `LD_LIBRARY_PATH`, you can configure the search path
globally by running:
```bash
$ sudo echo "/opt/papi/5.7.0/" > /etc/ld.so.conf.d/papi.conf
$ sudo ldconfig
```

## Platforms

The following platforms are currently tested:

* `x86_64-unknown-linux-gnu`, `rustc >= 1.36`, `gcc >= 4.8`, `clang >= 3.8`

