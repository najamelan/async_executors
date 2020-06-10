# async_executors - CHANGELOG

## 0.3.0 - 2020-06-10

  - update to async-std 1.6. Local spawn still requires the unstable feature on async-std.

## 0.3.0-beta.1 - 2020-05-10

  - futures 0.3.5 has been released, so we no longer have to vendor RemoteHandle. It means we are now `forbid( unsafe )` and
    the spawn_handle feature has been removed since we no longer require extra dependencies in order to provide the traits.
  - update to async-std 1.6.0-beta.1. Async-std is now backed by smol. It now supports Wasm and `LocalSpawn`. Local spawning
    is unstable, so we turn on the unstable feature. As they are still in beta, we reflect that in our version.


## 0.2.2 - 2020-04-25

  - Temporarily remove forbid unsafe. A non-breaking change update of pin-utils now causes a macro to no longer pass.
    We have this because we vendor RemoteHandle from futures until they release a new version.
  - Improve an error message for a panic on JoinHandle.


## 0.2.1 - 2020-04-08

  - FIX: JoinHandle::detach didn't work properly. Sorry, my bad as it wasn't even tested.
  - remove the Unpin impl from JoinHandle. The joinhandle is still Unpin anyway.
  - run cargo deny in CI.
  - Vamp up the docs, removing some errors and adding examples.
  - `TokioCt` and `TokioTp` block_on no longer require `&mut self`, just `&self`. Since they
    implement `Clone`, it didn't protect against re-entrance anyway.
  - improve performance of `spawn_handle_local` on `TokioCt` as I mistakenly thought `tokio::JoinHandle<T>`
    required `T` to be `Send`, so I was not using the native `JoinHandle`.
  - only build on default target on docs.rs.
  - clean up and configure the CI configuration.

## 0.2 - 2020-02-29

  - tracing-futures 0.2.3 is out, so no patching required anymore.

  - RemoteHandle is still vendored until the next release of futures.

  - TokioCt now uses tokio::task::LocalSet, removing the single line of unsafe we had.
    This also improves performance dramatically. Thanks @Yandros for pointing out
    LocalSet.

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

## 0.1.0 - 2020-02-24

  - Re-add TokioHandle to work around the fact that tokio::Runtime can't be dropped in async context.

## 0.1.0.alpha.2 - 2020-02-19

  - fix error in feature name.
  - clean up readme.

## 0.1.0.alpha.1 - 2020-02-19

This is an alpha release because:

  - tracing integration should be tested
  - examples for tracing/bindgen/spawn_handle/using futures executors
  - remote_handle currently vendored until: https://github.com/rust-lang/futures-rs/pull/2081 lands
  - tracing-futures currently patched until 0.2.3 to get the needed PR's
  - no cross framework support for spawn_blocking yet.
  - wasm-bindgen-cli does not compile on windows, breaking CI: https://github.com/rustwasm/wasm-bindgen/issues/2006



