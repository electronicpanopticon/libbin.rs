# libbin.rs

Template for Rust lib/bin module with built-in GitHub Action to build and test.

## Getting Started

You will want to change the lib name and bin name in Cargo.toml:

```toml
[lib]
name = "changemelib"
path = "src/lib.rs"

[[bin]]
name = "changeme"
path = "src/main.rs"
```

Then you will want to change first line in your main.rs to match the name you've
given the lib in your `Cargo.toml` file:

```rust
use changemelib::*;
```

## Dependencies

* [clap - Command Line Argument Parser for Rust](https://github.com/clap-rs/clap)

## Clippy

This template includes a
[rust-clippy-check](https://github.com/marketplace/actions/rust-clippy-check)
GitHub Action. You can adjust the threshold it fails on in the
`./github/workflow/rust.yml` file.

## Using the GitHub CLI

With the [GitHub CLI](https://cli.github.com/) you can create a respository
locally and on GitHub with a single command:

```
$❯ gh repo create foobinlib --template  https://github.com/electronicpanopticon/libbin.rs.git
```

## Running 

```
$❯ cargo run -- -b
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/changeme -b`
Boop!
```

```
$❯ cargo run -- -n "my friend"
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/changeme -n 'my friend'`
Hello, my friend!
```

## etc...

* Includes a
  [rust-toolchain](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)
  file to specify specific versions of rust in rustup.
* Includes a
  [.cargo/config.toml file](https://doc.rust-lang.org/cargo/reference/config.html)
  specifying a Rust version of at least 1.54.0 because of the use of
  [Clap](https://github.com/clap-rs/clap).
* Includes a
  [clippy.yml](https://github.com/rust-lang/rust-clippy#user-content-configuration)
<<<<<<< Updated upstream
  file to do custom linting configurations.
* Includes a
  [.rustfmt.toml](https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=)
  file to configure formatting.
=======
  config file to do custom configurations.

## Useful Info

* [Cheap tricks for high-performance Rust](https://deterministic.space/high-performance-rust.html)
>>>>>>> Stashed changes
