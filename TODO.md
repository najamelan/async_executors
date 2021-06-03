# TODO:

- Create features and traits for turning on extra executor global side-effects, like GlommioIO, TokioIO, AsyncStdIO, Timer.
- Test IO traits.
- Document IO traits usage.
- check documentation for glommio and new traits.
- async-global also supports tokio_io
- clarify in docs and cargo.yml that it is LocalSpawner that is the executor and not LocalPool.
- currently we don't require the Timer::SleepFuture to be Send on wasm. Is this the best approach? SendWrapper is
  another approach, but that feels wrong too as these days you can have threads with webworkers on wasm too.
- async-io also provides a timer. So maybe in async-global-executor if async-io is already enabled, we should use
  that instead of futures-timer.
- clarify in docs how to avoid adding +SpawnHandle<SomeOut> become a breaking change in your API. That it is better to take a trait alias.
- consistency of block_on fn, vs run_until. Probably add block_on to local spawner.
- verify unwind safety of all our public types and make sure the traits are correctly implemented or not.

- test for JoinHandle being Send when Out is Send. Currently was caught just by an example.
- test what happens when creating 2 LocalExecutor in one thread.
- glommio's CPU pinning.

- should LocalSpawnHandle imply SpawnHandle? If you can spawn a !Send future, normally you can always spawn a Send one.
  It would mean that API's that take in a LocalSpawnHandle can also use spawn_handle. Eg. nursery can impl Nurse also on
  an executor that is `impl LocalSpawnHandle<T>`. What does futures do with Spawn and LocalSpawn?

- support smolscale?

- wrapping the executors of the futures library would make it easier to interop with TokioCt if they were wrapped and we put block_on on the wrapper for consistent api. For running entire test suits on different executors for example. That is because with tokio ct you have to call block_on.

- think about timers and timeout.

- spawn_blocking? This is provided by tokio and async_std, but does not take a future, rather a closure.
  However it still returns a joinhandle that must be awaited. So if we wrap that in our joinhandle type,
  we now have inconsistent behavior, as both frameworks don't provide any way to cancel the closure when
  the joinhandle get's dropped. We could make a JoinBlocking type?

# Wrap up

- CI - fix windows wasm testing


## Consistent behavior:

The anwser to each point here needs to be the same for all supported executors and the ones from the futures library.

### Spawning

  ✔ what happens if the spawned future panics?

    ✔ on all executors except tokio, the executor thread will unwind. Tokio uses catch_unwind and
      the fact that the future panicked can only be observed by awaiting the joinhandle, but
      the Spawn and LocalSpawn traits do not return a JoinHandle, so there is no way to tell.

  ✔ is spawning fallible or infallible?
     We turn everything to fallible in line with the futures executors.

### JoinHandle

  ✔ provide both detach and drop.
  ✔ what happens if the joinhandle get's dropped.
  ✔ what happens if the future panics.

    ✔ remote handle unwinds the thread that is awaiting the handle
    ✔ async_std unwinds both
    ✔ tokio unwinds none.

    I brought tokio in line with remote_handle, but async_std will unwind the executor thread. This inconsistency remains.
