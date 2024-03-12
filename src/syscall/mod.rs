mod fs;
mod process;

use fs::*;
use process::*;

const SYS_OPENAT: usize             = 56;
const SYS_CLOSE: usize              = 57;
const SYS_LSEEK: usize              = 62;
const SYS_READ: usize               = 63;
const SYS_WRITE: usize              = 64;

const SYS_EXIT: usize               = 93;
const SYS_SCHED_YIELD: usize        = 124;
const SYS_GETTIMEOFDAY: usize       = 169;

pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    match syscall_id {
        SYS_OPENAT              => sys_openat(), 
        SYS_CLOSE               => sys_close(),
        SYS_LSEEK               => sys_lseek(),
        SYS_READ                => sys_read(),
        SYS_WRITE               => sys_write(),

        SYS_EXIT                => sys_exit(),
        SYS_SCHED_YIELD         => sys_sched_yield(),
        SYS_GETTIMEOFDAY        => sys_gettimeofday(),

        _                       => panic!("Unsupport syscall_id: {}", syscall_id),
    }
}
