use core::mem::size_of;
use alloc::string::String;

#[derive(Debug)]
pub struct DirectoryInfo{
    pub block_idx : usize,
    pub device_id : usize,
    pub dir_num : usize,
    pub file_num : usize,
    pub name_len : usize,
    pub struct_size : usize,
}

impl DirectoryInfo {
    pub fn get_name(&self, n : usize)->String {
        let mut s = String::new();
        let ptr = self as *const Self as usize;
        let ptr = (ptr + size_of::<Self>()) as *const char;
        let ptr = unsafe {ptr.add(n * self.name_len)};
        let mut idx = 0;
        while idx < self.name_len {
            let c = unsafe {*(ptr.add(idx))};
            if c == '\0' {
                break;
            }
            s.push(c);
            idx += 1;
        }
        s
    }
}

