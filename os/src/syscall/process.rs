//! Process management syscalls
use core::ops::BitAnd;
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
    },
};


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    return get_time_us() as isize;
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?

/// YOUR JOB: Finish sys_task_info to pass testcases
use crate::task::TASK_MANAGER;
use crate::timer::get_time_ms;

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // summary syscall
    let (status, syscall_times, create_time) = TASK_MANAGER.get_task_crate_time();
    unsafe {
        (*ti).status = status;
        // Correcting the logic to set the time as the duration since creation.
        let elapsed_time = get_time_ms() - create_time;
        // Assuming 'time' is a field that can hold a duration or timestamp.
        (*ti).time = elapsed_time;
        // Assuming `syscall_times` is a field that can be copied or assigned directly from `curr_task_info`.
        (*ti).syscall_times = syscall_times;
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let flag = port as u8;
    let mut mem_permission = MapPermission::empty();
    if flag & 0x2 != 0 {
        mem_permission = mem_permission.bitand(MapPermission::R);
    }
    if flag & 0x4 != 0 {
        mem_permission = mem_permission.bitand(MapPermission::W);
    }
    if flag & 0x8 != 0 {
        mem_permission = mem_permission.bitand(MapPermission::X);
    }
    if flag & 0x10 != 0 {
        mem_permission = mem_permission.bitand(MapPermission::U);
    }
    let mut kernel = KERNEL_SPACE.exclusive_access();

    kernel.insert_framed_area(
        start.into(),
        len.into(),
        mem_permission,
    );
    1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    KERNEL_SPACE.exclusive_access().shrink_to(start.into(), len.into());
    1
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}


use crate::timer::get_time_us;
use crate::mm::{
     MapPermission,  KERNEL_SPACE,
};