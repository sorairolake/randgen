<!--
SPDX-FileCopyrightText: 2025 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# randgen

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
![License][license-badge]

**randgen** is a tool which generates random bytes using a pseudorandom number
generator.

## Installation

### From source

```sh
cargo install randgen
```

### From binaries

The [release page] contains pre-built binaries for Linux, macOS and Windows.

### How to build

Please see [BUILD.adoc].

## Usage

### Basic usage

Generate 1 KiB of random bytes:

```sh
randgen 1KiB
```

### Generate shell completion

`--generate-completion` option generates shell completions to standard output.

The following shells are supported:

- `bash`
- `elvish`
- `fish`
- `nushell`
- `powershell`
- `zsh`

Example:

```sh
randgen --generate-completion bash > randgen.bash
```

## Command-line options

Please see the following:

- [`randgen(1)`]

## Source code

The upstream repository is available at
<https://github.com/sorairolake/randgen.git>.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright (C) 2025 Shun Sakai (see [AUTHORS.adoc])

1.  This program is distributed under the terms of either the _Apache License
    2.0_ or the _MIT License_.
2.  Some files are distributed under the terms of the _Creative Commons
    Attribution 4.0 International Public License_.

This project is compliant with version 3.2 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/randgen/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/randgen/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/randgen?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/randgen
[msrv-badge]: https://img.shields.io/crates/msrv/randgen?style=for-the-badge&logo=rust
[license-badge]: https://img.shields.io/crates/l/randgen?style=for-the-badge
[release page]: https://github.com/sorairolake/randgen/releases
[BUILD.adoc]: BUILD.adoc
[`randgen(1)`]: docs/man/man1/randgen.1.adoc
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
