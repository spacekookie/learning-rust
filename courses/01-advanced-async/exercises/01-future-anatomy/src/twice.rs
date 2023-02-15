use core::{future::Future,pin::Pin,task::{Context, Poll}};

pub struct Twice(bool);
impl Twice {
    pub fn new() -> Self {
        Self(false)
    }
}

impl Future for Twice {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            dbg!(Poll::Ready(()))
        } else {
            self.0 = true;
            dbg!(Poll::Pending)
        }
    }
}
