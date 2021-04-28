//! # 天目文件系统格式实现
//! 
//! 2021年4月13日 zg


use tianmu_fs::SuperBlock;
#[allow(dead_code)]
pub struct TianMu{
    pub device_id : usize,
    pub total_size : usize,
    pub block_size : usize,
    pub block_num : usize,
    pub block_map_addr : usize,
    pub root_idx : usize,
}

impl TianMu {
    pub fn new(device_id: usize, sp : SuperBlock)->Self {
        let setting = sp.setting;
        Self {
            device_id,
            total_size: (setting.block_num * setting.bytes_per_block) as usize,
            block_size: setting.bytes_per_block as usize,
            block_num: setting.block_num as usize,
            block_map_addr: setting.block_map_offset as usize,
            root_idx: setting.root_table_offset as usize / setting.bytes_per_block as usize,
        }
    }

    
}


