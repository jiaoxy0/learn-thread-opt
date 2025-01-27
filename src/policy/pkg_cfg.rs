use super::policy_unname;
use crate::policy::name_match::{policy_mingchao, policy_unity};
use compact_str::CompactString;
use hashbrown::HashMap;
use libc::pid_t;
use std::sync::LazyLock;

const UNITY: [&str; 7] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
    "com.papegames.infinitynikki",
];

const UNNAME: [&str; 3] = [
    "com.tencent.tmgp.pubgmhd",
    "com.netease.yyslscn",
    "com.miHoYo.hkrpsg",
];

const MINGCHAO: [&str; 3] = [
    "com.kurogame.mingchao",
    "com.kurogame.mingchao",
    "com.kurogame.mingchao",
];

type ConfigTuple<'a> = (&'a [&'a str], fn(&HashMap<pid_t, CompactString>));
pub static PACKAGE_CONFIGS: LazyLock<[ConfigTuple; 3]> = LazyLock::new(|| {
    [
        (&UNITY[..], policy_unity::start_task),
        (&UNNAME[..], policy_unname::start_task),
        (&MINGCHAO[..], policy_mingchao::start_task),
    ]
});
