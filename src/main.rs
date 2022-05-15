use async_psp::{task::Task, Executor};

fn main() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(async {
        println!("the magic number is: {}", async_number().await);
    }));
    executor.run();
    println!("done");
}
async fn async_number() -> u32 {
    42
}
