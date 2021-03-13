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

## Using the GitHub CLI

With the [GitHub CLI](https://cli.github.com/) you can create a respository
locally and on GitHub with a single command:

```
$❯ gh repo create foobinlib --template  https://github.com/electronicpanopticon/libbin.rs.git
```

## TODO

Make this template unnecessary by adding the functionality to
[`cargo new`](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html).
Right now you get the following error:

```
$❯ cargo new foo --bin --lib
error: can't specify both lib and binary outputs
```
