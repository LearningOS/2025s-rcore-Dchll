//! Process management syscalls
use crate::syscall::sys_id_trance;
use crate::task::get_sys_call_count;
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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

// implement the syscall
/// 追踪当前任务系统调用的历史信息
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let ptr = id as *const u8;
            if ptr.is_null() {
                return -1;
            }
            unsafe { *ptr as isize }
        }
        1 => {
            let ptr = id as *mut u8;
            if ptr.is_null() {
                return -1;
            }
            unsafe {
                *ptr = data as u8;
            }
            0
        }
        2 => {
            let i = sys_id_trance(id);
            get_sys_call_count(i)
        }
        _ => -1,
    }
}
