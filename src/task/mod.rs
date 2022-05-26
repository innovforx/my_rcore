mod context;

mod switch;

#[allow(clippy::module_inception)]
mod task;




use crate::{sync::UPSafeCell, config::MAX_APP_NUM};
use task::*;
use self::switch::switch;

use super::loader::{get_app_num,init_app_cx};
use lazy_static::lazy_static;
use context::*;


pub struct TaskManager{
    num_app : usize,
    inner : UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner{
    tasks : [TaskControlBlock;MAX_APP_NUM],
    current_task : usize,
}

lazy_static!{
    pub static ref TASK_MANAGER : TaskManager = {
        let num_app = get_app_num() ;
        let mut tasks = [TaskControlBlock{
            task_cx : TaskContext::zero_init(),
            task_status : TaskStatus::UnInit,
        };MAX_APP_NUM];

        for (i , t) in tasks.iter_mut().enumerate(){
            t.task_cx = TaskContext::goto_restore(init_app_cx(i));
            t.task_status = TaskStatus::Ready;
        }


        TaskManager { num_app: (num_app),
             inner: 
                unsafe {
                    UPSafeCell::new(TaskManagerInner { 
                        tasks: (tasks), 
                        current_task: (0) })
                },       
        
        }
        // TaskManager { num_app: (0), inner: (0) } 
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as * const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe{
            switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }

        panic!("unreachable code");
    }

    fn mark_current_suspend(&self){
        let mut inner = self.inner.exclusive_access();
        let current_tsk_id = inner.current_task;
        inner.tasks[current_tsk_id].task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self){
        let mut inner = self.inner.exclusive_access();
        let current_tsk_id = inner.current_task;
        inner.tasks[current_tsk_id].task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize>{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1 .. current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }


    fn run_next_app(&self){
        if let Some(next) = self.find_next_task(){
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_cx as *mut TaskContext;

            drop(inner);
            unsafe{
                switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        }else{
            panic!("all complated");
        }
    }

}


pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}

fn run_next_app(){
    TASK_MANAGER.run_next_app();
}

fn mark_current_exited(){
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_suspend(){
    TASK_MANAGER.mark_current_suspend();
}


pub fn suspend_current_and_run_next(){
    TASK_MANAGER.mark_current_suspend();
    run_next_app();
}


pub fn exit_current_and_run_next(){
    TASK_MANAGER.mark_current_exited();
    run_next_app();
}







