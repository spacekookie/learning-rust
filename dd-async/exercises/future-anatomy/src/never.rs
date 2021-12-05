use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct Never;

impl Future for Never {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}
