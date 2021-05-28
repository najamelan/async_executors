use async_executors::GlommioCt;
use {
    futures::channel::{oneshot, oneshot::Sender},
    futures::task::{Spawn, SpawnExt},
};

fn lib_function(exec: impl Spawn, tx: Sender<&'static str>) {
    exec.spawn(async {
        tx.send("I can spawn from a library").expect("send string");
    })
    .expect("spawn task");
}

fn main() {
    // You provide the builder, and async_executors will set the right scheduler.
    // Of course you can set other configuration on the builder before.
    //
    let exec = GlommioCt::new("unnamed", None);

    let program = async {
        let (tx, rx) = oneshot::channel();

        lib_function(&exec, tx);
        println!("{}", rx.await.expect("receive on channel"));
    };

    exec.block_on(program);
}
