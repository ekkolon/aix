# aix

> This project is still in the experimental stage, and its functionality may change at any time without notice.

A CLI tool to quickly scaffold REST API projects based on the [Actix](https://actix.rs/) framework.

Projects generated with `aix` come with reasonable defaults and can be extended with [_extras_](#extras) to further enhance your development activities.

## Installation

aix is published as both a binary and library crate on [crates.io](https://crates.io/aix). For now, the aix library is just published for completeness.

**Install aix binary**:

```bash
cargo install aix
```

**Add aix to a project**:

```bash
cargo add aix
```

## Usage

```console
Usage: aix [OPTIONS] <COMMAND>

Commands:
  new
          Generates a new Rust + Actix starter project
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Sets the level of verbosity (-v, -vv, -vvv)
  -h, --help
          Print help
  -V, --version
          Print version
```

## Extras

Extras are opt-in setups for adding common development workflows and utilities to projects generated with the **aix** CLI.

### Available extras

- **Docker**

  Adds a `Dockerfile` and `.dockerignore` to the generated project.

- **CI** (GitHub Actions)

  Adds a basic `ci.yml` GitHub Actions workflow that contains jobs to _lint_ | _test_ | _build_ your project

## License

Licensed under either of

- [MIT license](https://spdx.org/licenses/MIT.html) (see [LICENSE-MIT](/LICENSE-MIT)) or
- [Apache License, Version 2.0](https://spdx.org/licenses/Apache-2.0.html) (see [LICENSE-APACHE](/LICENSE-APACHE))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
