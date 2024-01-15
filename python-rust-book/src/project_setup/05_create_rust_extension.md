# Setup Rust extension

We will now add a Rust extension to our project. We will use [maturin](https://www.maturin.rs/) to build the Rust extension. Maturin is a command-line tool to build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as Python packages.

You can easily add a Rust extension to your project with the following command:

```bash
poetry run maturin new rust-pidigits
```

This will create a new Rust extension called `rust-pidigits` in the `rust-pidigits` directory. The `rust-pidigits` directory will contain the following files:

```txt
├── rust-pidigits
│   ├── Cargo.toml
|   ├── Cargo.lock
│   ├── pyproject.toml
│   └── src
│       └── lib.rs

```

We can also remove the `pyproject.toml` file because we will use the `pyproject.toml` file in the root of the project. Change `build-system` table in `pyproject.toml` to:

```toml
[build-system]
requires = ["maturin>=1.4,<2.0"]
build-backend = "maturin"
```

Add the following table to the `pyproject.toml` file to configure maturin:

```toml
[tool.maturin]
# The name of the crate
name = "rust-pidigits"
# The version of the crate
version = "0.1.0"
# The name of the Python module
module = "rust_pidigits"
# The path to the Rust source code
path = "pidigits-rust/src/lib.rs"
# The minimum Python version required to use the crate
requires-python = ">=3.11"
# The Rust features to build with
features = ["pyo3/extension-module"]
description = "Calculate pi to the Nth digit"
readme = "README.md"
license = { file = "LICENSE.txt" }
# Remove this if you don't have clang and lld installed
# You can install clang and lld with `sudo apt install clang lld`
rustc-args = [
    "-C",
    "linker=clang",
    "-C",
    "link-arg=-fuse-ld=lld",
    "-C",
    "target-cpu=native",
]
manifest-path = "rust-pidigits/Cargo.toml"
profile = "release"
strip = true
codegen-units = 1
lto = true
```

```admonish info title="Maturin's configuration"
You can read more about maturin's configuration [here](https://www.maturin.rs/config)
```

```admonish info title="Metadata"
We can also add more Metadata to the `pyproject.toml` file: check out the specification from [here](https://packaging.python.org/en/latest/specifications/pyproject-toml/#pyproject-toml-spec)
```

For example, we can add the following Metadata to the `pyproject.toml` file:

```toml
[project]
name = "pidigits"
keywords = ["pi", "pi to the nth digit"]
authors = [
    { email = "mikko.leppanen@vaisala.com" },
    { name = "Mikko Leppänen" },
]
maintainers = [
    { name = "Mikko Leppänen", email = "mikko.leppanen@vaisala.com" },
]
classifiers = [
    "Programming Language :: Python",
    "Programming Language :: Rust",
    "Development Status :: 4 - Beta",
    "License :: OSI Approved :: MIT License",
    "Operating System :: POSIX :: Linux",
    "Intended Audience :: Developers",
]
```

Then the final thing we need to do is to add a couple of Rust dependencies to the `Cargo.toml` file:

- [rug](https://crates.io/crates/rug)
- [pyo3](https://crates.io/crates/pyo3)

```bash
cargo add rug -F float,integer
cargo add pyo3
```

This will add the following dependencies to the `Cargo.toml` file (your versions may differ):

```toml
[dependencies]
pyo3 = "0.20.2"
rug = { version = "1.22.0", features = ["float", "integer"] }
```

Now we are ready to start actually writing some Python&Rust code! 🎉