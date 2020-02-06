# async_executors

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_executors.svg?branch=master)](https://travis-ci.org/najamelan/async_executors)
[![Docs](https://docs.rs/async_executors/badge.svg)](https://docs.rs/async_executors)
[![crates.io](https://img.shields.io/crates/v/async_executors.svg)](https://crates.io/crates/async_executors)


> Abstract over different executors.

The aim of _async_executors_ is to provide a uniform interface to the main async executors available in Rust. We provide wrapper types that always implement the Spawn and/or LocalSpawn traits from the future library, making it easy to pass any executor to an API which requires `E: Spawn` or `E: LocalSpawn`. All executors require a `&mut` reference for spawning,
so all provide a cheap `Clone` implementation so you can keep passing them around.

Two new traits are introduced by this crate: `SpawnHandle` and `LocalSpawnHandle`. These provide methods that allow
spawning tasks while obtaining a `JoinHandle<T>` which allows awaiting the spawned task as well as recovering an output that is not `()`. The traits aren't object safe, and can't be without boxing the future.

The currently supported executors are:

- futures-rs LocalPool (support spawning `!Send` futures)
- futures-rs ThreadPool
- tokio CurrentThread (support spawning `!Send` futures)
- tokio ThreadPool
- async-std
- juliex
- wasm-bindgen (only available on WASM, the others are not available on WASM)

All executors are behind feature flags: `localpool`, `threadpool`, `tokio_ct`, `tokio_tp`, `async_std`, `juliex`, `bindgen`.

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


## Performance

Most wrappers are very thin and shouldn't add any performance degradation, however for most executors `JoinHandle<T>` is currently implemented with a oneshot channel because of limitations in existing implementations. Both the `remote_handle`
and the `JoinHandle` from the tokio threadpool have `Send` bounds that would have to be put on our `JoinHandle` meaning we would need a second type for thread local spawning. Further testing, benchmarking and discussion will be required to see if there is better ways of implementing `JoinHandle`.

Existing benchmarks for all executors can be found in [executor_benchmarks](https://github.com/najamelan/executor_benchmarks).

## Supported Executors

### Futures LocalPool

Simple executor that allows spawning `!Send` futures on the local thread. After spawning, you need to call one of the `run` methods to make it do any work. That will block the current thread and poll the futures. The wrapper type in this crate forwards all 4 run methods. Check the API docs of the futures library for more info.

This holds a `futures::executor::LocalPool` inside, so a `std::convert::From` impl is provided if you already have a `LocalPool`.

### Futures ThreadPool

Threadpool executor. It works very well for simple scenarios, but does not have a fancy workstealing scheduler. In contrast to the tokio threadpool, it seems that tasks keep running even if you drop the executor.

This holds a `futures::executor::ThreadPool` inside, so a `std::convert::From` impl is provided if you already have a `ThreadPool`. This allows you to configure the threadpool. See the futures-rs documentation for more information.

### Tokio CurrentThread

A single threaded executor that can spawn `!Send` tasks. You need to call `run` on it after spawning, which will block the current thread and poll the futures.


## Limitations

The current implementation of `JoinHandle<T>` is very much a proof of concept right now. For most executors it's implemented with a simple `oneshot::channel`. It's probably not the best for performmance (although I haven't tested that), but mainly, there is no way to cancel tasks. Dropping the `JoinHandle<T>` will just detach the task and it will keep running. There are other tools, like `remote_handle` from the futures library, but that requires a `Send` bound on the return type so it can't be used for all executors. The same goes for the joinhandle from tokio, and only the tokio thread_pool and not the current_thread executor returns a joinhandle.

The implementation will be improved over time, and is open for discussion.


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

