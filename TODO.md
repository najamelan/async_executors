# TODO:

## JoinHandle part

- check consistency: see below

- spawn_blocking?


# Wrap up

- tracing test/example
- wasm example
- blanket impls on ?Sized?
- Spawn::status
- documentation
- write blogpost

## Consistent behavior:

The anwser to each point here needs to be the same for all supported executors and the ones from the futures library.

### Spawning

  - what happens if the spawned future panics?
    - tokio uses catch_unwind, so it can only be observed in the output of the Joinhandle
    - async_std: doesn't catch_unwind, from the docs:
      //! Fatal logic errors in Rust cause *thread panic*, during which a thread will unwind the stack,
      //! running destructors and freeing owned resources. If a panic occurs inside a task, there is no
      //! meaningful way of recovering, so the panic will propagate through any thread boundaries all the
      //! way to the root task. This is also known as a "panic = abort" model.

    - futures: doesn't seem to call catch_unwind except in RemoteHandle.

    - bindgen: doesn't seem to call catch_unwind.

  ✔ is spawning fallible or infallible?
    We turn everything to fallible in line with the futures executors.

### JoinHandle

  ✔ provide both detach and drop.
  ✔ what happens if the joinhandle get's dropped.
  - what happens if the future panics.



assert unwindsafe:
- what does UnwindSafe trait do when encountered in an unwind?
- do tools only deal with unwind safety if they deal with threading? See LocalPool, async-task single thread example?
- Send + 'static != UnwindSafe, why does tokio not use the trait from stdlib?
- list of ressources:
 - nomicon: https://doc.rust-lang.org/nomicon/unwinding.html#unwinding
 - std docs for:
   - UnwindSafe: https://doc.rust-lang.org/std/panic/trait.UnwindSafe.html
   - AssertUnwindSafe
   - std::panic
 - RFC: https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md
 - Issue:
