# async_executors

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_executors.svg?branch=master)](https://travis-ci.org/najamelan/async_executors)
[![Docs](https://docs.rs/async_executors/badge.svg)](https://docs.rs/async_executors)
[![crates.io](https://img.shields.io/crates/v/async_executors.svg)](https://crates.io/crates/async_executors)


> Abstract over different executors.

The aim of _async_executors_ is to provide a uniform interface to the main async executors available in Rust. We provide wrapper types that always implement the [`Spawn`](futures_util::task::Spawn) and/or [`LocalSpawn`](futures_util::task::LocalSpawn) traits from _futures_, making it easy to pass any supported executor to an API which requires `exec: impl Spawn` or `exec: impl LocalSpawn`.

A problem with these traits is that they do not provide an API for getting a `JoinHandle` to await completion of the task. The current trend in async is to favor joining tasks and thus certain executors now return `JoinHandle`s from their spawn function. It's also a fundamental building block for [structured concurrency](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/). _Async_executors_ provides an executor agnostic [`JoinHandle`] that wraps the framework native JoinHandle types.

`SpawnHandle` traits are also provided so API's can express you need to pass them an executor which allows spawning whilst returning a `JoinHandle`. Note that this was already provided by the _futures_ in [`SpawnExt::spawn_with_handle`](futures_util::task::SpawnExt::spawn_with_handle), but this uses [`RemoteHandle`](futures_util::future::RemoteHandle) which incurs a performance overhead. By wrapping the native JoinHandle types, we can avoid some of that overhead while still being executor agnostic.

The traits provided by this crate are also implemented for the [`Instrumented`](tracing_futures::Instrumented) and [`WithDispatch`](tracing_futures::WithDispatch) wrappers from _tracing-futures_ when the `tracing` feature is enabled. So you can pass an instrumented executor to an API requiring `exec: impl SpawnHandle<SomeOutput>`.

The currently supported executors are (file an issue on GitHub if you want to see another one supported):

- [async-global-executor](https://docs.rs/async-global-executor) - supports spawning `!Send` futures and works on Wasm.
- [async-std](https://docs.rs/async-std) - supports spawning `!Send` futures and works on Wasm (uses async-global-executor and bindgen under the hood).
- [tokio](https://docs.rs/tokio) CurrentThread - [`tokio::runtime::Runtime`] with basic scheduler and a LocalSet. (supports spawning `!Send` futures)
- [tokio](https://docs.rs/tokio) ThreadPool - [`tokio::runtime::Runtime`] with threadpool scheduler.
- [glommio](https://docs.rs/glommio) Glommio is a Cooperative Thread-per-Core crate for Rust & Linux based on io_uring.
- [wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures) (only available on Wasm)
- the [futures-executor](https://docs.rs/futures-executor) executors - They already implemented `Spawn` and `SpawnLocal`, but we implement the `SpawnHandle` family of traits for them as well. The types `ThreadPool`, `LocalPool` and `LocalSpawner` are re-exported for convenience.

All executors are behind feature flags: `async_std`, `async_global`, `tokio_ct`, `tokio_tp`, `glommio`, `bindgen`, `localpool`, `threadpool`.


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

   async_executors: ^0.4
```

With Cargo.toml
```toml
[dependencies]

    async_executors = "0.4"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_executors/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

The only hard dependencies are `futures-task` and `futures-util`. The rest are the optional dependencies to turn on support for each executor.

## Security

The crate itself uses `#[ forbid(unsafe_code) ]`.

Our dependencies use unsafe.


## Performance

Most wrappers are very thin but the `Spawn` and `LocalSpawn` traits do imply boxing the future. With executors boxing futures
to put them in a queue you probably get 2 heap allocations per spawn.

`JoinHandle` uses the native `JoinHandle` types from _tokio_ and _async-std_ to avoid the overhead from `RemoteHandle`, but for _async-std_, wrap the future in `Abortable` to create consistent behavior across all executors. The `JoinHandle` provided cancels it's future on drop unless you call `detach` on it.

`SpawnHandle` and `LocalSpawnHandle` require boxing the future twice, just like `Spawn` and `LocalSpawn`.

Existing benchmarks for all executors can be found in [executor_benchmarks](https://github.com/najamelan/executor_benchmarks).


## Missing features

These are some features that aren't provided yet but that are on the todo list:

- an agnostic timeout mechanism.
- an agnostic interface for `spawn_blocking`.


## Usage

### For API providers

When writing a library that needs to spawn, you probably shouldn't lock your clients into one framework or another. It's usually not appropriate to setup your own thread pool for spawning futures. It belongs to the application developer to decide where futures are spawned and it might not be appreciated if libraries bring in extra dependencies on a framework.

In order to get round this you can take an executor in as a parameter from client code and spawn your futures on the provided executor. Currently the only two traits that are kind of widely available for this are `Spawn` and `LocalSpawn` from the _futures_ library. Unfortunately, other executor providers do not implement these traits. So by publishing an API that relies on these traits, you would have been restricting the clients to use the executors from _futures_, or start implementing their own wrappers that implement the traits.

_Async_executors_ has wrappers providing impls on various executors, namely _tokio_, _async_std_, _wasm_bindgen_. As such you can just use the trait bounds and refer your users to this crate if they want to use any of the supported executors.

All wrappers also implement `Clone`, `Debug` and the zero sized ones also `Copy`. You can express you will need to clone in your API: `impl Spawn + Clone`.

Note that you should never use `block_on` inside async contexts. Some backends we use like _tokio_ and `RemoteHandle` from _futures_ use `catch_unwind`, so try to keep futures unwind safe.

#### Spawning with handles

You can use the `SpawnHandle` and `LocalSpawnHandle` traits as bounds for obtaining join handles.

##### Example

```rust
use
{
  async_executors :: { JoinHandle, SpawnHandle, SpawnHandleExt       } ,
  std             :: { sync::Arc                                     } ,
  futures         :: { FutureExt, executor::{ ThreadPool, block_on } } ,
};


// Example of a library function that needs an executor. Just use impl Trait.
//
fn needs_exec( exec: impl SpawnHandle<()> )
{
   let handle = exec.spawn_handle( async {} );
}


// A type that needs to hold on to an executor during it's lifetime. Here it
// must be heap allocated.
//
struct SomeObj{ exec: Arc< dyn SpawnHandle<u8> > }


impl SomeObj
{
   pub fn new( exec: Arc< dyn SpawnHandle<u8> > ) -> SomeObj
   {
      SomeObj{ exec }
   }

   fn run( &self ) -> JoinHandle<u8>
   {
      self.exec.spawn_handle( async{ 5 } ).expect( "spawn" )
   }
}


fn main()
{
  let exec = ThreadPool::new().expect( "build threadpool" );
  let obj  = SomeObj::new( Arc::new(exec) );

  let x = block_on( obj.run() );

  assert_eq!( x, 5 );
}
```


### For API consumers

You can basically pass the wrapper types provided in _async_executors_ to API's that take any of the following. Traits are also implemented for `Rc`, `Arc`, `&`, `&mut`, `Box` and `Instrumented` and `WithDispatch` from _tracing-futures_ wrappers:

  - `impl Spawn`
  - `impl LocalSpawn`
  - `impl SpawnHandle<T>`
  - `impl LocalSpawnHandle<T>`

All wrappers also implement `Clone`, `Debug` and the zero sized ones also `Copy`.

Some executors are a bit special, so make sure to check the API docs for the one you intend to use. Some also provide extra methods like `block_on` which will call a framework specific `block_on` rather than the one from _futures_.

#### Example

```rust
use
{
  async_executors :: { AsyncStd, TokioTpBuilder, SpawnHandle } ,
  std             :: { convert::TryFrom                      } ,
};

fn needs_exec( exec: impl SpawnHandle<()> + SpawnHandle<String> ){};

// AsyncStd is zero sized, so it's easy to instantiate.
//
needs_exec( AsyncStd );

let tp = TokioTpBuilder::new().build().expect( "build threadpool" );

needs_exec( tp );
```

For more examples, check out the [examples directory](https://github.com/najamelan/async_executors/tree/master/examples). If you want to get a more polished API for adhering to structured concurrency, check out [_async_nursery_](https://crates.io/crates/async_nursery).

## API

API documentation can be found on [docs.rs](https://docs.rs/async_executors).


## Contributing

Please check out the [contribution guidelines](https://github.com/najamelan/async_executors/blob/master/CONTRIBUTING.md).


### Testing

Run `ci/test.bash` and `ci/wasm.bash` to run all tests.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](https://github.com/stumpsyn/policies/blob/master/citizen_code_of_conduct.md#4-unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

