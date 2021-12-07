use core::{future::Future,pin::Pin,task::{Context, Poll}};

pub struct Twice(bool);
impl Twice {
    pub fn new() -> Self {
        Self(false)
    }
}

impl Future for Twice {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            ctx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
