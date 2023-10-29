//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_current_task_info,
        mmap, munmap, suspend_current_and_run_next, update_task_info, TaskStatus,
    },
    timer::get_time_us,
};

use crate::config::PAGE_SIZE;

use crate::mm::page_table::translated_refmut;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    update_task_info(crate::syscall::SYSCALL_EXIT);

    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    update_task_info(crate::syscall::SYSCALL_YIELD);

    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    update_task_info(crate::syscall::SYSCALL_GET_TIME);

    let us = get_time_us();
    *translated_refmut(current_user_token(), ts) = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    update_task_info(crate::syscall::SYSCALL_TASK_INFO);

    *translated_refmut(current_user_token(), ti) = get_current_task_info();

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    update_task_info(crate::syscall::SYSCALL_MMAP);

    if _start % PAGE_SIZE != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }

    mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    update_task_info(crate::syscall::SYSCALL_MUNMAP);

    if _start % PAGE_SIZE != 0 {
        return -1;
    }

    munmap(_start, _len)
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    update_task_info(crate::syscall::SYSCALL_SBRK);

    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
