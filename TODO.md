# TODO:

- spawn_handle and spawn_handle_local. For now uses remote_handle from futures-rs. How does this compare to a oneshot::channel? It requires Send on the output type. Should we make traits out of these? Or just remove them and tell people to use remote_handle themselves or a channel?

- Testing: right now, we have basic testing of the functionality, but we should go over everything thinking about what might go wrong and test edge cases/verify our tests cover everything.
  - test/benchmark SendHandle on LocalPool
  - test tokio-fs and stuff that needs a reactor

- impl tokio Executor and TypedExecutor traits for all executors.

- threadpool, tokio::threadpool, juliex, (async-std, if they can make a standalone executor)
