# Specifics

A simple desktop calculator for calculating rent in our household.



Defaults are:
It calculates the total amount, based on what percentage of each participants share of "avdrag

`cargo run` to run it.

# Slint Rust Template

A template for a Rust application that's using [Slint](https://slint-ui.com) for the user interface.

## About

This template helps me get started developing a Rust application with Slint as toolkit
for the user interface. It demonstrates the integration between the `.slint` UI markup and
Rust code, how to trigger react to callbacks, get and set properties and use basic widgets.

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
   ```
   cargo install cargo-generate
   ```
3. Set up a sample project with this template
   ```
   cargo generate --git https://github.com/slint-ui/slint-rust-template --name my-project
   cd my-project
   ```
4. Build with cargo
   ```
   cargo build
   ```
5. Run the application binary
   ```
   cargo run
   ```

Its recommended using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

Automate inputs from bills or payment plans. 