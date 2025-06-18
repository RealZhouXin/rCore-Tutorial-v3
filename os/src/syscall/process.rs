//! App management syscalls
use crate::batch::{get_current_app, run_next_app};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
pub fn sys_print_task_info() -> isize {
    println!("[kernel] task id: {}", get_current_app());
    0
}
