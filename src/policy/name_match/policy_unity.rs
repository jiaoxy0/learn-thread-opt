use crate::policy::{name_match::common::Policy, pkg_cfg::StartArgs};
#[cfg(debug_assertions)]
use log::debug;
use std::time::Duration;

const TOP: [&str; 0] = [];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

pub fn start_task(args: &mut StartArgs) {
    loop {
        let pid = args.activity_utils.top_app_utils.get_pid();
        if pid != args.pid {
            return;
        }
        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();
        let task_map = args.activity_utils.tid_utils.get_task_map(*pid);
        Policy::new(&TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND).execute_policy(task_map);
        #[cfg(debug_assertions)]
        {
            let end = start.elapsed();

            debug!(
                "单线程:一轮绑定核心完成时间: {:?} 数组长度{}",
                end,
                task_map.len()
            );
        }
        std::thread::sleep(Duration::from_millis(2000));
    }
}
