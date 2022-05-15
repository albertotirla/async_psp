use super::task::Task;
use alloc::collections::VecDeque;

pub struct Executor {
    task_queue: VecDeque<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }
}