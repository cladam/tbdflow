## Installation

You need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### Installing from crates.io

The easiest way to install `tbdflow` is to download it from [crates.io](https://crates.io/crates/tbdflow). You can do it using the following command:

```bash
cargo install tbdflow
```

If you want to update `tbdflow` to the latest version, execute the following command:

```bash
tbdflow update
```

### Building from source

Alternatively you can build `tbdflow` from source using Cargo:

```bash
git clone https://github.com/cladam/tbdflow.git
cd tbdflow
sudo cargo install --path . --root /usr/local
```

