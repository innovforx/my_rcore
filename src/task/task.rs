//! Types related to task management

use core::fmt::Display;

use crate::{timer::get_time_ms, mm::{MemorySet, PhyAddr, PhysPageNum, VirtAddr, KERNEL_SPACE, MapPermission}, trap::{TrapContext, trap_handler}, config::{TRAP_CONTEXT, kernel_stack_position}, Debugln};

use super::{TaskContext, info::TaskInfo};

// #[derive(Copy, Clone)]
#[derive(Debug)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info : TaskInfo,
    pub memory_set : MemorySet,
    pub trap_cx_ppn : PhysPageNum,
    pub base_size : usize,
}

#[derive(Copy, Clone, PartialEq,Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskControlBlock {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext{
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize{
        self.memory_set.token()
    }

    pub fn new(elf_data : &[u8],app_id : usize) -> Self{
        let (memory_set,user_sp,entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap().ppn();

        let task_status = TaskStatus::Ready;

        let (kernel_stack_bottom,kernel_stack_top) = kernel_stack_position(app_id);
        // Debugln!("mao dian 1");
        KERNEL_SPACE.exclusive_access().insert_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W);
        let task_control_block = Self{
            task_status,
            task_cx : TaskContext::goto_trap_return(kernel_stack_top),
            memory_set,
            trap_cx_ppn,
            base_size:user_sp,
            task_info : TaskInfo::zero_init(),
        };
        // Debugln!("mao dian 2");
        let trap_cx = task_control_block.get_trap_cx();

        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
             kernel_stack_top,
              trap_handler as usize);
        // Debugln!("mao dian 3");
        // Debugln!("{:?}",trap_cx);
        task_control_block

    }



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