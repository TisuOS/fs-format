//! # 文件信息
//!
//! 2021年4月29日 zg

#[derive(Debug)]
pub struct FileInfo {
    pub id : usize,
    pub device_id : usize,
    pub start_idx : usize,
    pub name : *const char,
    pub path : *const char,
    pub flag : usize,
    pub size : usize,
}

impl Clone for FileInfo {
    fn clone(&self) -> Self {
        Self {
            id:self.id,
            device_id:self.device_id,
            start_idx:self.start_idx,
            name:self.name,
            path:self.path,
            flag:self.flag,
            size:self.size,
        }
    }
}

impl FileInfo {
    pub fn new(
        id:usize,
        device_id:usize,
        start_idx:usize,
        name:*const char,
        path:*const char,
        flag:usize,
        size:usize,
    )->Self {
        Self {
            id,
            device_id,
            start_idx,
            name,
            path,
            flag,
            size,
        }
    }

}
