use crate::types::{VideoFrame, ProcessingConfig};
use image::{Frame, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::path::Path;

pub struct VideoDerivativeProcessor {
    previous_frame: Option<VideoFrame>,
    config: ProcessingConfig,
    frame_counter: usize,
}

impl VideoDerivativeProcessor {
    pub fn new(config: ProcessingConfig) -> Self {
        Self {
            previous_frame: None,
            config,
            frame_counter: 0,
        }
    }
    pub fn process_frame(&mut self, frame: &VideoFrame) -> VideoFrame {
        let derivative_data = match &self.previous_frame {
            Some(prev_frame) => {
                if self.config.noise_reduction {
                    self.compute_thresholded_difference(&prev_frame.data, &frame.data)

                }else{
                    self.compute_thresholded_difference(&prev_frame.data, &frame.data)
                }
            }
            None =>{
                ImageBuffer::new(frame.width(), frame.height())
            }
        };
        let derivative_frame = VideoFrame::new(
            derivative_data, self.frame_counter, frame.timestamp
        );

        self.previous_frame = Some(frame.clone());
        self.frame_counter+=1;
        derivative_frame
    }
    pub fn compute_frame_difference(&self, frame1: &RgbImage, frame2: &RgbImage) -> RgbImage {
        let (width, height) = (frame1.width(), frame2.height());
        let mut derivative = RgbImage::new(width, height);

        for (x, y, pixel) in derivative.enumerate_pixels_mut() {
            if x < width && y < height {
                let pixel1 = frame1.get_pixel(x, y);
                let pixel2 = frame2.get_pixel(x, y);

                let diff_r = (pixel1[0] as i16 - pixel2[0] as i16).abs() as u8;
                let diff_g = (pixel1[1] as i16 - pixel2[1] as i16).abs() as u8;
                let diff_b = (pixel1[2] as i16 - pixel2[2] as i16).abs() as u8;

                *pixel = Rgb([diff_r, diff_g, diff_b]);
            }
        }

        derivative
    }
    fn compute_thresholded_difference(&self, frame1: &RgbImage, frame2: &RgbImage) -> RgbImage {
        let (width, height) = (frame1.width(), frame2.height());
        let mut derivative = RgbImage::new(width, height);


        for (x, y, pixel) in derivative.enumerate_pixels_mut() {
            let pixel1 = frame1.get_pixel(x, y);
            let pixel2 = frame2.get_pixel(x, y);

            let diff_r = (pixel1[0] as i16 - pixel2[0] as i16).abs() as u8;
            let diff_g = (pixel1[1] as i16 - pixel2[1] as i16).abs() as u8;
            let diff_b = (pixel1[2] as i16 - pixel2[2] as i16).abs() as u8;

            let result_r = if diff_r>self.config.threshold{diff_r}else{0};
            let result_g = if diff_r>self.config.threshold{diff_g}else{0};
            let result_b = if diff_r>self.config.threshold{diff_b}else{0};

            *pixel = Rgb([result_r, result_g, result_b]);
        }
        derivative
    }
    pub fn reset(&mut self){
        self.previous_frame = None;
        self.frame_counter = 0;
    }
    pub fn get_config(&self)->&ProcessingConfig{
        &self.config
    }
    pub fn update_config(&mut self, config:ProcessingConfig){
        self.config = config;
    }
}