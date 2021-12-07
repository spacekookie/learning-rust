use ockam::{Context, Message, Result, Routed, Worker};
use serde::{Deserialize, Serialize};

struct Calculator;

#[derive(Serialize, Deserialize, Message)]
struct Sum(usize, usize);

#[derive(Serialize, Deserialize, Message)]
struct SumReply(usize);

#[ockam::worker]
impl Worker for Calculator {
    type Context = Context;
    type Message = Sum;

    async fn handle_message(&mut self, ctx: &mut Context, msg: Routed<Sum>) -> Result<()> {
        ctx.send(msg.return_route(), SumReply(msg.0 + msg.1)).await
    }
}

#[ockam::node]
async fn main(ctx: &mut Context) -> Result<()> {
    ctx.start_worker("calc", Calculator).await?;
    ctx.send("calc", Sum(10, 5)).await?;

    let res = ctx.receive::<SumReply>().await?;
    println!("Sum: {}", res.0);

    ctx.stop().await
}
