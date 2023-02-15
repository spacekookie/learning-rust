use core::{future::Future, pin::Pin, task::{Context, Poll}};

pub struct Always;

impl Future for Always {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        Poll::Ready(())
    }
}
