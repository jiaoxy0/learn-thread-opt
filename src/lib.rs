#![no_std]
#![no_main]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]

// 库 crate 根

// 模块树应该定义在 src/lib.rs 中
/*这样通过以包名开头的路径，公有项就可以在二进制
crate 中使用。二进制 crate 就变得同其它在该 crate 之外的、使用库 crate 的用户一
样：二者都只能使用公有 API。这有助于你设计一个好的 API；你不仅仅是作者，也
是用户！ */
mod activity;
mod cgroup;
mod config;
mod cpu_common;
mod misc;
mod policy;
mod scheduler;
mod utils;

use core::{ffi::CStr, slice};
use libc::{c_char, c_int};
use misc::init_misc;
use scheduler::Scheduler;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn affinity_setting_worker(argc: c_int, argv: *const *const c_char) {
    init_misc();
    let args = unsafe { slice::from_raw_parts(argv, argc.try_into().unwrap_or(0)) };

    for arg in args {
        if let Ok(s) = unsafe { CStr::from_ptr(*arg) }.to_str() {
            #[cfg(debug_assertions)]
            log::info!("命令行参数{s}");
        }
    }
    Scheduler::new().start_run();
}
