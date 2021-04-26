//! BMP
//! 提供 BMP 格式图片操控接口，仅通过容器与切片交互
//!
//! 2020年12月31日 zg

use tisu_driver::Pixel;

use super::{image::{ColorFormat, Image}};

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct BMP{
    btype : u16,
    size : u32,
    reserved : u32,
    data_offset : u32,
    info_size : u32,
    width : u32,
    height : i32,
    plane : u16,
    bitcnt : u16,
    compression_type : CompressType,
    image_size : u32,
    pixel_x : i32,
    pixel_y : i32,
    cnt_used : u32, // 彩色表中颜色索引数，为 0 使用所有
    important_cnt : u32, // 有重要影响的颜色索引数目
}

impl BMP {
    pub fn is_bmp(data : &[u8])->bool{
        data[0] == 0x42 && data[1] == 0x4d
    }

    pub fn decode(data : &[u8])->Result<Image, BmpError> {
        if !Self::is_bmp(data) {
            return Err(BmpError::NotBMP);
        }
        let bmp = unsafe {&*(data as *const [u8] as *const u8 as *const BMP)};
        let width = bmp.width as usize;
        let height = bmp.height.abs() as usize;
        let is_down = bmp.height > 0;
        let mut rt = Image::new(width, height, ColorFormat::RGB);
        let mut data_offset = bmp.data_offset as usize;
        let skip = (4 - (width * bmp.bitcnt as usize) % 4) % 4;
        for _ in 0..height{
            for _ in 0..width{
                // let color = *(ptr.add(data_offset) as *mut BMPColor);
                let b = data[data_offset];
                let g = data[data_offset + 1];
                let r = data[data_offset + 2];
                let a = data[data_offset + 3];
                rt.add(Pixel{r, g, b, a});
                data_offset += 4;
            }
            data_offset += skip;
        }
        if is_down {
            rt.flip();
        }

        Ok(rt)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BmpError {
    NotBMP,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct BMPColor{
    b:u8,
    g:u8,
    r:u8,
    a:u8
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressType{
    RGB = 0,
}
impl CompressType{
    pub fn is_compressed(&self)->bool{
        *self != CompressType::RGB
    }
}





