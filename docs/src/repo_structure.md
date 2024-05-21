# Repo structure

The structure of the `statue` dir:

* `src/`: Source code.
* `target/`: Compiled code (generated).
* `.gitingore`: [Git ignore file](https://git-scm.com/docs/gitignore).
* `.markdownlint.json`: Configuration file for Markdown linting, used by [`DavidAnson.vscode-markdownlint`](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint) Visual Studio Code extension.
* `Cargo.toml` and `Cargo.lock` : Manifest and machine-generated list of dependencies. [Learn more](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
* `LICENSE-APACHE: Apache License, Version 2.0.
* `LICENSE-MIT: MIT License.
* `README.md: The README for the crate.

The structure of the `docs` dir:

* `src/`: the source code for the mdbook.
* `book.toml`: the configuration file for the mdbook.
* `book`: the compiled mdbook (generated).
* `.gitingore`: [Git ignore file](https://git-scm.com/docs/gitignore).

Also,

* `.github/workflows`: GitHub Actions workflows.
