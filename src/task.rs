
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
///a task represents an async operation being spawned on an executor
///when an asyncronous operation is wrapped in this task type and spawned, it will run concurrently in the executor untill it completes
/// such an async operation is represented by an asyncronous function, containing one or more await points which transfer controll to other futures the operation depends on
///usually, async operations include file i/o, communication over the network, basically anything that's io bound
pub struct Task {
    operation: Pin<Box<dyn Future<Output = ()>>>,
}
impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            operation: Box::pin(future),
        }
    }
    fn poll(&mut self, ctx: &mut Context) -> Poll<()> {
        self.operation.as_mut().poll(ctx)
    }
}
