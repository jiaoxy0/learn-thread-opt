use super::policy_common::{execute_task, get_cmd_type};
use libc::pid_t;

const TOP: [&str; 1] = [" "];
const ONLY6: [&str; 1] = ["UnityGfxDeviceW"];
const ONLY7: [&str; 1] = ["UnityMain"];
const MIDDLE: [&str; 2] = ["Thread-", "Job.Worker"];
const BACKEND: [&str; 0] = [];

pub fn start_task(tid: pid_t, thread: &str) {
    let thread_type = get_cmd_type(thread, &TOP, &ONLY6, &ONLY7, &MIDDLE, &BACKEND);
    execute_task(&thread_type, tid);
}
