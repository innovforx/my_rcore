

use crate::{syscall::{SYSCALL_NUM, get_syscall_name}, timer::get_time_ms};
use super::task::TaskStatus;
use core::fmt::Display;

#[derive(Copy,Clone,Debug)]
pub struct TaskInfo{
    pub id :usize,
    pub name : Option<&'static str>,
    pub status : TaskStatus,
    call : [SyscallInfo;SYSCALL_NUM],
    pub time : usize,
}
#[derive(Copy,Clone,Debug)]
struct SyscallInfo{
    id : usize,
    times : usize,
}


impl TaskInfo {
    pub fn new(id : usize,name : Option<&'static str>,status : TaskStatus,time : usize) -> Self{
        
        TaskInfo { id: (id),name : (name), status: (status), call: [SyscallInfo::new();SYSCALL_NUM], time: (time) }
    }
    pub fn zero_init() -> Self{
        TaskInfo { id: (0), name: (None), status: (TaskStatus::UnInit), call: ([SyscallInfo::new();SYSCALL_NUM]), time: (0) }
    }
    pub fn get_syscall_cnt(&self,syscall_id : usize) -> usize{
        for si in & self.call{
            if si.id == syscall_id {
                return si.times;
            }
        }
        0
    }
    pub fn syscall_cnt_add(&mut self,syscall_id : usize,cnt : usize) {
        for si in &mut self.call{
            if si.id == syscall_id {
                si.times += cnt;
                return;
            }
        }
        self.add_new_syscall_stat(syscall_id,cnt)        
    }

    pub fn add_new_syscall_stat(&mut self,syscall_id : usize,cnt : usize){
        for si in &mut self.call{
            if(si.id == 0){
                si.id = syscall_id;
                si.times = cnt;
                return;                
            }
        }
    }

    pub fn set_time(&mut self,time : usize){
        self.time = time;
    }

    pub fn set_start_time(&mut self){
        if self.time == 0{
            self.time = get_time_ms();
        }
    }
}

impl Display for TaskInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"id : {},name : {},status : {}\n\trunning time : {}\n"
            ,self.id
            ,match  self.name {
                Some(na) => na,
                None => "No name",
            } 
            ,self.status
            ,get_time_ms() - self.time
            );
        for sys_stat in &self.call{
            if sys_stat.id == 0{
                break;
            }
            write!(f,"\t{}\n",sys_stat);
        }
        write!(f,"")
        
    }
}


impl SyscallInfo {
    pub fn new() -> Self{
        SyscallInfo { id: (0), times: (0) }
    }
}

impl Display for SyscallInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"type : {},cnt : {}",get_syscall_name(self.id),self.times)
    }
}