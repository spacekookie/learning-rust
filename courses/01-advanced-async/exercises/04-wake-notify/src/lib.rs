use core::task::Waker;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Clone)]
pub struct Notify<T> {
    inner: T,
    waker: Option<Waker>,
}

impl<T> Deref for Notify<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Notify<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.waker.as_ref().map(|w| w.wake_by_ref());
        &mut self.inner
    }
}

impl<T> Notify<T> {
    pub fn new(inner: T) -> Self {
        Self { inner, waker: None }
    }

    pub fn wake(ptr: &Notify<T>) {
        if let Some(ref w) = ptr.waker {
            w.wake_by_ref();
        }
    }

    pub fn add_waker(ptr: &mut Notify<T>, w: Waker) {
        ptr.waker = Some(w);
    }
}
