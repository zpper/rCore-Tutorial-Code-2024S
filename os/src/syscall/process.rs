//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM},
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::get_time_us,
};
use crate::timer::get_time_ms;

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
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}


/// get sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // summary syscall
    let (status, syscall_times, create_time) = TASK_MANAGER.get_task_crate_time();
    unsafe {
        (*ti).status = status;
        // Correcting the logic to set the time as the duration since creation.
        let elapsed_time = get_time_ms() - create_time;
        (*ti).time = elapsed_time; // Assuming 'time' is a field that can hold a duration or timestamp.

        // Assuming `syscall_times` is a field that can be copied or assigned directly from `curr_task_info`.
        (*ti).syscall_times = syscall_times;
    }
    // *(&ti).status = *status;
    // (&ti).time.get_time_ms() - create_time;
    // (&ti).syscall_times = curr_task_info;
    0
}
