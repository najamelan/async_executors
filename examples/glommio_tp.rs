use
{
    futures::task::{Spawn, SpawnExt},
    futures::channel::{oneshot, oneshot::Sender},
};
use async_executors::GlommioTpBuilder;
use std::sync::Arc;


fn lib_function(exec: impl Spawn, tx: Sender<&'static str>)
{
    exec.spawn(async
        {
            tx.send("I can spawn from a library").expect("send string");
        }).expect("spawn task");
}


fn main()
{
    // You provide the builder, and async_executors will set the right scheduler.
    // Of course you can set other configuration on the builder before.
    //
    let builder = GlommioTpBuilder::new(2);
    let exec = builder.build().unwrap();
    let exec_ = Arc::clone(&exec);
    let program = async
        {
            let (tx, rx) = oneshot::channel();

            lib_function(exec_, tx);
            println!("{}", rx.await.expect("receive on channel"));
        };
    exec.block_on(program);
}
