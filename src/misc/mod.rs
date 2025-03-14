pub mod logger;
use crate::{cgroup::group_info::print_group_core, utils::node_reader::write_to_byte};
use itoa::Buffer;
use libc::getpid;
use likely_stable::unlikely;
use log::info;
use logger::{init_log, log_metainfo};

/// 初始化杂项：
/// 设置本线程后台运行权限、初始化tklog日志库、设置主线程名字、输出版本信息日志、输出杂项日志、输出CPU信息日志
pub fn init_misc() {
    working_in_background();
    init_log();
    set_main_thread_name(b"AffinitySetter\0");
    log_metainfo();
    print_misc();
    print_group_core();
}

/// 把本线程的pid转换成C字符串，写入/dev/cpuset/background/tasks文件中
/// 涉及c库函数：getpid()
fn working_in_background() {
    unsafe {
        let pid = getpid();
        let mut itoa_buf = Buffer::new();
        let pid = itoa_buf.format(pid).as_bytes();
        let _ = write_to_byte(b"/dev/cpuset/background/tasks\0", pid);
    }
}

/// 将主线程的名称设置为AffinitySetter
/// 涉及c库函数：pthread_self() pthread_setname_np()
fn set_main_thread_name(name: &[u8]) {
    let thread_name = if unlikely(name.len() > 15) {
        &name[..15]
    } else {
        name
    };

    unsafe {
        libc::pthread_setname_np(libc::pthread_self(), thread_name.as_ptr());
    }
}

/// 输出一些log
fn print_misc() {
    info!("免费软件，禁止商用");
    info!("Free software, not for commercial use.");
    info!("开源地址: https://github.com/reigadegr/thread-opt");
}
