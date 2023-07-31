use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use once_cell::sync::Lazy;
use pin_project_lite::pin_project;
use tokio::runtime::{Builder, Runtime};

#[cfg(not(target_family = "wasm"))]
pub static RT: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name("lebai-sdk")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap()
});

pub trait CompatExt {
    fn compat(self) -> Compat<Self>
    where
        Self: Sized;
    fn compat_ref(&self) -> Compat<&Self>;
    fn compat_mut(&mut self) -> Compat<&mut Self>;
}

impl<T> CompatExt for T {
    fn compat(self) -> Compat<Self>
    where
        Self: Sized,
    {
        Compat::new(self)
    }
    fn compat_ref(&self) -> Compat<&Self> {
        Compat::new(self)
    }
    fn compat_mut(&mut self) -> Compat<&mut Self> {
        Compat::new(self)
    }
}

pin_project! {
pub struct Compat<T> {
    #[pin]
    inner: T,
}
}

impl<T> Compat<T> {
    pub fn new(t: T) -> Compat<T> {
        Compat { inner: t }
    }
}

impl<T: Future> Future for Compat<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        #[cfg(not(target_family = "wasm"))]
        let _guard = RT.enter();
        self.project().inner.poll(cx)
    }
}
