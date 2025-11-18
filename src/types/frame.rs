use std::process::Output;
use image::{ImageBuffer, Rgb, RgbImage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFrame{
    pub data: RgbImage,
    pub timestamp:f64,
    pub frame_number:usize
}
impl VideoFrame{
    pub fn new (data:RgbImage, frame_number:usize, timestamp:f64)-> Self{
        Self{
            data, frame_number, timestamp
        }
    }
    pub fn width(&self)->u32{
        self.data.width()
    }
    pub fn height(&self)->u32{
        self.data.height()
    }
    pub fn dimension(&self)->(u32, u32){
        (self.data.width(), self.data.height())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig{
    pub threshold:u8,
    pub fps: u32,
    pub output_width:u32,
    pub output_height: u32,
    pub noise_reduction:bool
}
impl Default for ProcessingConfig{
    fn default() -> Self {
        Self{
            threshold:32,
            fps: 32,
            output_width:640,
            output_height:480,
            noise_reduction:true
        }
    }
}