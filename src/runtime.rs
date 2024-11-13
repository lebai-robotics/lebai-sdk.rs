use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use pin_project_lite::pin_project;

#[cfg(all(not(target_family = "wasm"), feature = "module"))]
pub static RT: once_cell::sync::Lazy<tokio::runtime::Runtime> = once_cell::sync::Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("lebai-sdk")
        .worker_threads(1)
        .thread_keep_alive(core::time::Duration::MAX)
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

impl<T: Future> CompatExt for T {
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
        #[cfg(all(not(target_family = "wasm"), feature = "module"))]
        let _guard = RT.enter();
        self.project().inner.poll(cx)
    }
}
