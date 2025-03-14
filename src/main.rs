#![no_std]
#![no_main]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

// 二进制 crate 根

mod activity;
mod cgroup;
mod config;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;
// 二进制crate为什么也要声明模块树？
// 为啥没见二进制crate根用到库crate中的模块和

use core::{ffi::CStr, slice};
use libc::{c_char, c_int};
use misc::init_misc;
use scheduler::Scheduler;

// 二进制 crate 中只保留足以生成一个可执行文件的代码，并由可执行文件调用库 crate 的代码

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[unsafe(no_mangle)]
unsafe extern "C" fn main(argc: c_int, argv: *const *const c_char) {
    // 初始化杂项
    init_misc();

    // 处理命令行参数
    let args = unsafe { slice::from_raw_parts(argv, argc.try_into().unwrap_or(0)) };

    for arg in args {
        if let Ok(s) = unsafe { CStr::from_ptr(*arg) }.to_str() {
            #[cfg(debug_assertions)]
            log::info!("命令行参数{s}");
        }
    }

    // run
    Scheduler::new().start_run();
}
