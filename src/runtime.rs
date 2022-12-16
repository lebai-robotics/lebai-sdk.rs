use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

pub static RT: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("lebai-sdk")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
});
