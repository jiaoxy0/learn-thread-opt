use crate::{
    cgroup::group_info::{get_background_group, get_middle_group, get_top_group},
    utils::affinity_setter::{bind_thread_to_cpu, bind_tid_list_to_cgroup},
};
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
#[cfg(debug_assertions)]
use log::debug;

// 定义线程类型
enum CmdType {
    Only6,
    Only7,
}

// 执行策略
pub fn execute_policy(task_map: &HashMap<pid_t, CompactString>, first: pid_t, second: pid_t) {
    execute_task(&CmdType::Only7, first);
    execute_task(&CmdType::Only6, second);

    let filtered_keys: Vec<pid_t> = task_map
        .keys()
        .filter(|&&tid| tid != first && tid != second)
        .copied()
        .collect();

    let background_group = get_background_group();
    let middle_group = get_middle_group();

    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    if background_group == middle_group {
        bind_tid_list_to_cgroup(middle_group, &filtered_keys);
    } else {
        let new_array = [background_group, middle_group].concat();
        bind_tid_list_to_cgroup(&new_array, &filtered_keys);
    }

    #[cfg(debug_assertions)]
    {
        let end = start.elapsed();

        debug!("一轮绑定核心完成时间: {:?} 数组长度{}", end, task_map.len());
    }
}

// 执行线程绑定任务
fn execute_task(cmd_type: &CmdType, tid: pid_t) {
    match cmd_type {
        CmdType::Only6 => {
            let top_group = get_top_group();
            if top_group == [6, 7] {
                bind_thread_to_cpu(&[6], tid);
                return;
            }
            bind_thread_to_cpu(get_middle_group(), tid);
        }
        CmdType::Only7 => bind_thread_to_cpu(&[7], tid),
    }
}
