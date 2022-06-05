mod context;
mod info;
mod switch;

#[allow(clippy::module_inception)]
mod task;




use crate::{sync::UPSafeCell, loader::{get_app_data,}, println, timer::get_time_ms, Errorln, trap::TrapContext, Debugln};
use alloc::vec::Vec;
use task::*;
use self::switch::switch;

use super::loader::{get_num_app};
use lazy_static::lazy_static;
use context::*;
use info::TaskInfo;

pub struct TaskManager{
    num_app : usize,
    inner : UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner{
    tasks : Vec<TaskControlBlock>,
    current_task : usize,
}

lazy_static!{
    pub static ref TASK_MANAGER : TaskManager = {
        let num_app = get_num_app() ;
        println!("num app {}",num_app);
        let mut tasks : Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app{
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }
        Debugln!("TaskManager init success");
        TaskManager { 
            num_app,
             inner: unsafe{
                UPSafeCell::new(TaskManagerInner{
                    tasks,
                    current_task : 0,
                })
             }
                
        } 
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.set_status(TaskStatus::Running);        
        let next_task_cx_ptr = &task0.task_cx as * const TaskContext;

        let next_task_info = &mut task0.task_info;
        next_task_info.set_start_time();

        drop(inner);
        let mut _unused = TaskContext::zero_init();
        Debugln!("ready to switch to first");
        unsafe{
            switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }

        panic!("unreachable code");
    }
    
    fn mark_current_suspend(&self){
        let mut inner = self.inner.exclusive_access();
        let current_tsk_id = inner.current_task;
        inner.tasks[current_tsk_id].set_status(TaskStatus::Ready);
    }

    fn mark_current_exited(&self){
        let mut inner = self.inner.exclusive_access();
        let current_tsk_id = inner.current_task;
        let current_tcb = &mut  inner.tasks[current_tsk_id];
        current_tcb.set_status(TaskStatus::Exited);
        println!("{}",current_tcb.task_info)
    }

    fn find_next_task(&self) -> Option<usize>{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1 .. current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn get_current_token(&self) -> usize{
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_user_token()
    }

    fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].get_trap_cx()
    }

    fn run_next_app(&self){
        if let Some(next) = self.find_next_task(){
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].set_status(TaskStatus::Running);
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &mut inner.tasks[next].task_cx as *mut TaskContext;
            let next_task_info = &mut inner.tasks[next].task_info;
            next_task_info.set_start_time();
            drop(inner);
            unsafe{
                switch(current_task_cx_ptr, next_task_cx_ptr);
            }
        }else{
            panic!("all complated");
        }
    }

    fn current_app_syscall_stat_add_on(&self,syscall_id : usize){
        let mut inner = self.inner.exclusive_access();
        let current_idx = inner.current_task;
        let current_app_info = &mut inner.tasks[current_idx].task_info;
        current_app_info.syscall_cnt_add(syscall_id,1);
    }

    fn get_task_info(&self,id : isize)->Option<TaskInfo>{
        let inner = self.inner.exclusive_access();
        let current_id;
        if id < 0 {
            current_id = inner.current_task;
        }else{
            current_id = id as usize;
        }
        let task_info = inner.tasks[current_id].task_info.clone();

        Some(task_info)
        
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

pub fn update_current_app_syscall_info(syscall_id : usize){
    TASK_MANAGER.current_app_syscall_stat_add_on(syscall_id);
}

pub fn get_current_task_info()-> TaskInfo{
    match TASK_MANAGER.get_task_info(-1) {
        Some(i) => i,
        None => {
            Errorln!("sys error ,exit and run next");
            exit_current_and_run_next();
            panic!("unreachable")
        }
    }
}

pub fn current_app_token() -> usize{
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext{
    TASK_MANAGER.get_current_trap_cx()
}





