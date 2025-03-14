use super::analysis::{BACKEND_GROUP, MIDDLE_GROUP, TOP_GROUP};
use log::info;

pub fn get_top_group() -> &'static [u8] {
    &TOP_GROUP
}

pub fn get_middle_group() -> &'static [u8] {
    &MIDDLE_GROUP
}

pub fn get_background_group() -> &'static [u8] {
    &BACKEND_GROUP
}

/// 把CPU核心信息输出到log
/// eg: 
/// [{time}] INFO: TOP_GROUP: [6, 7]
/// [{time}] INFO: MIDDLE_GROUP: [0, 1, 2, 3, 4, 5]
/// [{time}] INFO: BACKEND_GROUP: []
pub fn print_group_core() {
    let top_group = get_top_group();
    let middle_group = get_middle_group();
    let background_group = get_background_group();

    info!("TOP_GROUP: {:?}", top_group);
    info!("MIDDLE_GROUP: {:?}", middle_group);
    info!("BACKEND_GROUP: {:?}", background_group);
}
