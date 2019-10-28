# async_executors

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_executors.svg?branch=master)](https://travis-ci.org/najamelan/async_executors)
[![Docs](https://docs.rs/async_executors/badge.svg)](https://docs.rs/async_executors)
[![crates.io](https://img.shields.io/crates/v/async_executors.svg)](https://crates.io/crates/async_executors)


> Abstract over different executors.

Version 0.0.0! WIP, not functional yet.

The aim of _async_executors_ is to provide a uniform interface to the main executors available in Rust. We provide wrapper types that always implement the Spawn and/or SpawnLocal traits from the future library, making it easy to pass any executor to an API which requires `E: Spawn` or `E: SpawnLocal`.

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [Basic Example](#basic-example)
   - [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_executors`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   async_executors: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

    async_executors = "^0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_executors/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security




## Usage



### Basic example

```rust

```

## API

API documentation can be found on [docs.rs](https://docs.rs/async_executors).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through Github issues.

Pull Requests are welcome on Github. By committing pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.

Please file PR's against the `dev` branch, don't forget to update the changelog and the documentation.

### Testing


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

