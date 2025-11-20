use crate::types::VideoFrame;
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use rand::Rng;

pub struct FrameGenerator{
    width: u32,
    height: u32,
    fps: u32
}
impl FrameGenerator{
    pub fn new(width: u32, height: u32, fps: u32)->Self{
        Self{
            width, height, fps
        }
    }
    pub fn generate_moving_object_frames(&self, num_frames: usize)->Vec<VideoFrame>{

        let mut frames = Vec::with_capacity(num_frames);
        let mut rng = rand::thread_rng();
        for i in 0..num_frames{
            let timestamp = i as f64/ self.fps as f64;
            let mut frame = self.create_background();

            let object_x = (i as u32 * 8)%(self.width - 80);
            let object_y = 100 + (20.0 * (i as f32 * 0.05).sin()).abs() as u32;

            self.add_rectangle(&mut frame, object_x, object_y, 80, 80, [255, 255, 255]);

            if i %4 ==0 {
                self.add_random_noise(&mut frame, 15);
            }

            frames.push(VideoFrame::new(frame, i, timestamp));
        }
        frames
    }
    pub fn generate_complex_scene_frames(&self, num_frames: usize)-> Vec<VideoFrame>{
        let mut frames = Vec::with_capacity(num_frames);

        for i in 0..num_frames{
            let timestamp = i as f64/ self.fps as f64;
            let mut frame = self.create_gradient_background();

            //some go object
            let obj1_x = (i as u32 *5)% (self.width-50);
            let obj1_y = 50+(15.0*(i as f32*0.1).sin()).abs() as u32;

            self.add_rectangle(&mut frame, obj1_x, obj1_y, 50, 50, [255, 0, 0]);

            let obj2_x = (self.width - 60).saturating_sub((i as u32 *3)%(self.width-60));
            let obj2_y = 150+(10.0 * (i as f32 * 0.2).cos()).abs() as u32;

            self.add_circle(& mut frame, obj2_x, obj2_y, 25, [0, 255, 0]);

            frames.push(VideoFrame::new(frame, i, timestamp));
        }
        frames
    }
    fn create_background(&self)->RgbImage{
        ImageBuffer::from_fn(self.width, self.height, |x, y|{
            let r = (x as f32 *100.0/ self.width as f32) as u8;
            let g = (y as f32* 100.0/ self.height as f32)as u8;
            let b = 100;
            Rgb([r, g, b])
        })
    }
    fn create_gradient_background(&self)->RgbImage{
        ImageBuffer::from_fn(self.width, self.height, |x, y|{
            let r = (x as f32 *255.0/ self.width as f32) as u8;
            let g = (y as f32* 255.0/ self.height as f32)as u8;
            let b = ((x+y)as f32 * 255.0/(self.width+self.height)as f32)as u8;
            Rgb([r, g, b])
        })
    }
    fn add_rectangle(&self, frame: &mut RgbImage, x:u32, y:u32, width: u32, height: u32, color:[u8; 3]){
        for dx in 0..width{
            for dy in 0..height{
                let px = x+dx;
                let py = y+dy;
                if px< self.width &&py<self.height{
                    frame.put_pixel(px, py, Rgb(color));
                }
            }
        }
    }
    fn add_circle(&self, frame: &mut RgbImage, center_x: u32, center_y:u32, radius: u32, color: [u8; 3]){
        let r_squared = (radius*radius) as i32;


        for dx in -(radius as i32)..= radius as i32{
            for dy in -(radius as i32)..=radius as i32{
                let diff_x = center_x as i32- dx ;
                let diff_y = center_y as i32- dy;

                //i think that error
                if diff_x*diff_x+diff_y+diff_y<=r_squared{
                    let px = (center_x as i32 + dx) as u32;
                    let py = (center_y as i32 + dy) as u32;

                    if px<self.width &&py<self.height{
                        frame.put_pixel(px, py, Rgb(color))
                    }
                }
            }
        }

    }
    fn add_random_noise(&self, frame:&mut RgbImage, num_pixels:u32){
        let mut rng = rand::thread_rng();
        for _ in 0..num_pixels{
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.width);
            let r = rng.gen_range(0..=255);
            let r = rng.gen_range(0..=255);
            let g = rng.gen_range(0..=255);
            let b = rng.gen_range(0..=255);
            frame.put_pixel(x, y, Rgb([r,g, b]))
        }
    }
}