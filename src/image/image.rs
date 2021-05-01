//! 图片
//! 所有图片格式最终转换成此接口
//! 
//! 2020年12月31日 zg

use tisu_driver::Pixel;
use alloc::vec::Vec;


pub struct Image{
    pub width : usize,
    pub height : usize,
    pub format : ColorFormat,
    pub data : Vec<Pixel>
}

impl Image {
    pub fn new(width : usize, height : usize, format : ColorFormat)->Self {
        let mut data = Vec::new();
        data.reserve(width * height);
        Self {
            width,
            height,
            format,
            data,
        }
    }

    pub fn add(&mut self, color : Pixel) {
        self.data.push(color);
    }

    pub fn flip(&mut self) {
        for row in 0..self.height / 2 {
            let t1 = row * self.width;
            let t2 = (self.height - row - 1) * self.width;
            for col in 0..self.width {
                self.data.swap(t1 + col, t2 + col);
            }
        }
    }

    pub fn resize(&mut self, width : usize, height : usize) {
        let mut data = Vec::new();
        data.reserve(width * height);
        for y in 0..height {
            let yy = y * self.height / height * self.width;
            for x in 0..width {
                let xx = x * self.width / width;
                data.push(*self.data.get(xx + yy).unwrap());
            }
        }
        self.width = width;
        self.height = height;
        self.data = data;
    }
}

#[allow(dead_code)]
pub enum ColorFormat{
    RGB,
    RGBA,
}


