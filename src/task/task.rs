//! Types related to task management

use core::fmt::Display;

use crate::timer::get_time_ms;

use super::{TaskContext, info::TaskInfo};

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info : TaskInfo,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskControlBlock {
    pub fn set_status(&mut self,status : TaskStatus){
        self.task_status = status;
        self.task_info.status = status;
    }

    pub fn get_running_time_ms(&self) -> usize{
        let cur_time = get_time_ms();
        cur_time - self.task_info.time
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            TaskStatus::Exited => write!(f,"Exited"),
            TaskStatus::UnInit => write!(f,"UnInit"),
            TaskStatus::Ready => write!(f,"Ready"),
            TaskStatus::Running => write!(f,"Running"),
        }
    }
}