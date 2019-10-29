# Benchmarks

## current_thread

Compares futures::executor::LocalPool and tokio_executor::current_thread::CurrentThread.

- spawn a single future that awaits a send on an unbounded channel.
- spawn many futures that await a send on an unbounded channel.
- spawn nested futures

## Ring: concurrent message passing

Create N tasks in a ring. Let every task send a usize counter to the next until the message has come back to the initiating task. Each task should initiate, so that in total N^2 messages get send. Each task compares the counter. If it is N, they stop, if counter < N, they increment the counter and pass the message to the next task, so every message makes an entire round around the ring.

