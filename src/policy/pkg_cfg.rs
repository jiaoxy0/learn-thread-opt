use super::name_match::{policy_cocos, policy_ue, policy_unity};
use crate::{
    activity::ActivityUtils,
    cpu_common::Controller,
    policy::usage::{usage_top1::policy_ue_top1, usage_top2::policy_usage2},
};
use libc::pid_t;
use once_cell::sync::Lazy;

const UNITY: [&str; 6] = [
    "com.miHoYo.Yuanshen",
    "com.miHoYo.hkrpg",
    "com.tencent.tmgp.sgame",
    "com.miHoYo.Nap",
    "com.yongshi.tenojo.ys",
    "com.tencent.tmgp.speedmobile",
];

const UE_USAGE_T1: [&str; 3] = [
    "com.tencent.lzhx",
    "com.tencent.tmgp.pubgmhd",
    "com.tencent.tmgp.pubgmhd",
];

const USAGE_T2: [&str; 1] = ["com.netease.yyslscn"];

const UE: [&str; 3] = [
    "com.kurogame.mingchao",
    "com.papegames.infinitynikki",
    "com.kurogame.mingchao",
];

const COCOS: [&str; 1] = ["com.bf.sgs.hdexp"];

pub struct StartArgs<'a> {
    pub controller: &'a mut Controller,
    pub activity_utils: &'a mut ActivityUtils,
    pub pid: &'a pid_t,
}

type ConfigTuple<'a> = (&'a [&'a str], fn(&mut StartArgs));

pub static PACKAGE_CONFIGS: Lazy<[ConfigTuple; 5]> = Lazy::new(|| {
    [
        (&UE_USAGE_T1[..], policy_ue_top1::start_task),
        (&USAGE_T2[..], policy_usage2::start_task),
        (&UNITY[..], policy_unity::start_task),
        (&UE[..], policy_ue::start_task),
        (&COCOS[..], policy_cocos::start_task),
    ]
});
