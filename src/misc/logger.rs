// From shadow3aaa fas-rs
use log::info;
use tklog::{LEVEL, LOG};

/// 初始化tklog日志库，之后便可以用info!宏输出log
pub fn init_log() {
    let logger = LOG;
    logger.set_level(LEVEL::Debug);
    logger.set_formatter("[{time}] {level}: {message}\n");
    logger.uselog();
}

/// 用于输出一条log
pub fn log_metainfo() {
    info!(
        "thread-opt v{} {}, llvm-{}, rustc-{}, on {},{},{}",
        env!("CARGO_PKG_VERSION"),
        build_type(),
        env!("VERGEN_RUSTC_LLVM_VERSION"),
        env!("VERGEN_RUSTC_SEMVER"),
        env!("VERGEN_SYSINFO_NAME"),
        env!("VERGEN_SYSINFO_OS_VERSION"),
        env!("VERGEN_RUSTC_HOST_TRIPLE")
    );
}

const fn build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "Debug build"
    } else {
        "Release build"
    }
}
