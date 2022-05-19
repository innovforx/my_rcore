
use super::context::TaskContext;


#[derive(Debug,Clone, Copy)]
pub struct TaskControlBlock{
    pub task_status : TaskStatus,
    pub task_cx :  TaskContext,
}
#[derive(Debug,PartialEq)]
pub enum TaskStatus{
    UnInit,
    Ready,
    Running,
    Exited,
}