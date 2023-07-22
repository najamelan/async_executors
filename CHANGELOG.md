# async_executors - CHANGELOG


## [Unreleased]

[Unreleased]: https://github.com/najamelan/async_executors/compare/0.7.0...dev


## [0.7.0] - 2023-07-22

[0.7.0]: https://github.com/najamelan/async_executors/compare/0.6.0..0.7.0

### Added 

  - **BREAKING**: Create tokio executors from currently entered runtimes as well as handles.

### Removed 

  - **BREAKING**: The _tokio_ builder types. As tokio now exposes the `runtime_flavour`, we
    can use it to keep you from creating a runtime of the wrong type. Thus the builder types
    are no longer needed.
    
### Updated
  - **BREAKING**: glommio to v0.8

## [0.6.0] - 2022-04-24

[0.6.0]: https://github.com/najamelan/async_executors/compare/0.5.1..0.6.0

### Added 

  - **BREAKING**: update glommio to 0.7.


## [0.5.1] - 2022-01-06

[0.5.1]: https://github.com/najamelan/async_executors/compare/0.5.0..0.5.1

### Added 

  - forward SpawnBlocking trait from `tracing_futures::Instrumented` and `tracing_futures::WithDispatch`.


## [0.5.0] - 2022-01-04

[0.5.0]: https://github.com/najamelan/async_executors/compare/0.4.2..0.5.0

### Added 

  - **BREAKING**: update glommio to 0.6.
  - add `SpawnBlocking` trait and `BlockingHandle`.


## [0.4.2] - 2021-06-10

[0.4.2]: https://github.com/najamelan/async_executors/compare/0.4.1...0.4.2

### Fixed
  - feature `external_doc` is removed in rustdoc 1.54.

### Added
  - support for Glommio current thread executor. Thanks to @qiujiangkun.
  - `YieldNow` trait.
  - `SpawnBlocking` trait and `BlockingHandle`.
  - `Timer` trait for agnostic sleep and timeout. Thanks to @nmathewson for sharing
    their work on tor-rtcompat.
  - `TokioIo` trait.


## [0.4.1] - 2021-04-24

[0.4.1]: https://github.com/najamelan/async_executors/compare/0.4.0...0.4.1

### Fixed
  - a missing feature flag on futures-util.


## [0.4.0] - 2021-01-01

[0.4.0]: https://github.com/najamelan/async_executors/compare/0.4.0-beta.2...0.4.0

### Added
  - support for async-global-executor v2.
  - support for tokio v1


## [0.4.0-beta.2] - 2020-11-05

[0.4.0-beta.2]: https://github.com/najamelan/async_executors/compare/0.4.0-beta.1...0.4.0-beta.2

### Added
  - support for async-global-executor.


## [0.4.0-beta.1] - 2020-11-01

[0.4.0-beta.1]: https://github.com/najamelan/async_executors/compare/0.3.0...0.4.0-beta.1

### Added
  - BREAKING CHANGE: support tokio 0.3. Will go out of beta when tokio releases 1.0.
  - add example for use with tracing.

### Fixed
  - update cargo deny.


## [0.3.0] - 2020-06-10

[0.3.0]: https://github.com/najamelan/async_executors/compare/0.3.0-beta.1...0.3.0

### Fixed
  - update to async-std 1.6. Local spawn still requires the `unstable` feature on async-std so _async_executors_ enables that.

## [0.3.0-beta.1] - 2020-05-10

[0.3.0-beta.1]: https://github.com/najamelan/async_executors/compare/0.2.2...0.3.0-beta.1

### Fixed
  - futures 0.3.5 has been released, so we no longer have to vendor RemoteHandle. It means we are now `forbid( unsafe )` and
    the spawn_handle feature has been removed since we no longer require extra dependencies in order to provide the traits.
  - update to async-std 1.6.0-beta.1. Async-std is now backed by smol. It now supports Wasm and `LocalSpawn`. Local spawning
    is unstable, so we turn on the unstable feature. As they are still in beta, we reflect that in our version.


## [0.2.2] - 2020-04-25

[0.2.2]: https://github.com/najamelan/async_executors/compare/0.2.1...0.2.2

### Fixed
  - Temporarily remove forbid unsafe. A non-breaking change update of pin-utils now causes a macro to no longer pass.
    We have this because we vendor RemoteHandle from futures until they release a new version.
  - Improve an error message for a panic on JoinHandle.


## [0.2.1] - 2020-04-08

[0.2.1]: https://github.com/najamelan/async_executors/compare/0.2.0...0.2.1

### Fixed
  - JoinHandle::detach didn't work properly. Sorry, my bad as it wasn't even tested.
  - remove the Unpin impl from JoinHandle. The joinhandle is still Unpin anyway.
  - run cargo deny in CI.
  - Vamp up the docs, removing some errors and adding examples.
  - `TokioCt` and `TokioTp` block_on no longer require `&mut self`, just `&self`. Since they
    implement `Clone`, it didn't protect against re-entrance anyway.
  - improve performance of `spawn_handle_local` on `TokioCt` as I mistakenly thought `tokio::JoinHandle<T>`
    required `T` to be `Send`, so I was not using the native `JoinHandle`.
  - only build on default target on docs.rs.
  - clean up and configure the CI configuration.

## [0.2.0] - 2020-02-29

[0.2.0]: https://github.com/najamelan/async_executors/compare/0.1.0...0.2.0

### Fixed
  - **BREAKING CHANGE**: the API of SpawnHandle has been reworked. 0.1 had double traits, one
    not object safe. There were two reasons for this:
    - the os version needed an extra boxing. Benchmarks showed that the overhead from this is neglectable.
    - the os version needs to have the output type on the trait instead of on the spawn function.
      this is inconvenient if you need to take in an executor that can spawn for several output types, but
      since it is merely inconvenient, I feel this is not a good enough argument to have the traits in 2
      versions. Workaround examples have been added to the documentation of `SpawnHandle` and `LocalSpawnHandle`.

    The `SpawnHandle` trait now takes a `FutureObj` instead of a `Pin<Box<dyn Future>>`. This should be better
    for `no_std` compatibility. An extension trait has been added much like `SpawnExt` in the futures lib to
    allow spawning futures without having to manually create a `FutureObj`.

  - tracing-futures 0.2.3 is out, so no patching required anymore.
  - RemoteHandle is still vendored until the next release of futures.
  - TokioCt now uses tokio::task::LocalSet, removing the single line of unsafe we had.
    This also improves performance dramatically. Thanks @Yandros for pointing out
    LocalSet.

## [0.1.0] - 2020-02-24

[0.1.0]: https://github.com/najamelan/async_executors/compare/0.1.0-alpha.2...0.1.0

### Fixed
  - Re-add TokioHandle to work around the fact that tokio::Runtime can't be dropped in async context.

## 0.1.0-alpha.2 - 2020-02-19

  - fix error in feature name.
  - clean up readme.

## 0.1.0-alpha.1 - 2020-02-19

This is an alpha release because:

  - tracing integration should be tested
  - examples for tracing/bindgen/spawn_handle/using futures executors
  - remote_handle currently vendored until: https://github.com/rust-lang/futures-rs/pull/2081 lands
  - tracing-futures currently patched until 0.2.3 to get the needed PR's
  - no cross framework support for spawn_blocking yet.
  - wasm-bindgen-cli does not compile on windows, breaking CI: https://github.com/rustwasm/wasm-bindgen/issues/2006



