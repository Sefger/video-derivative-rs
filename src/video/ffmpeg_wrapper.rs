use crate::types::VideoFrame;
use image::{ImageBuffer, Rgb, RgbImage};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::fs;
use std::default::Default;
use tempfile::TempDir;


pub struct VideoProcessor {
    temp_dir: TempDir,
}

impl VideoProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        Ok(Self { temp_dir })
    }

    pub fn extract_frames_from_video(
        &self,
        video_path: &str,
        fps: Option<u32>,
    ) -> Result<Vec<VideoFrame>, Box<dyn std::error::Error>> {
        let frames_dir = self.temp_dir.path().join("extracted_frames");
        fs::create_dir_all(&frames_dir)?;

        let output_pattern = frames_dir.join("frame_%06d.png").to_string_lossy().to_string();

        let fps_arg = fps.map_or("".to_string(), |f| format!("-r {}", f));

        let status = Command::new("ffmpeg")
            .args(&[
                "-i", video_path,
                &fps_arg,
                "-vsync", "vfr",
                &output_pattern,
            ])
            .status()?;

        if !status.success() {
            return Err("Ошибка при извлечении кадров из видео".into());
        }
        let mut frames = Vec::new();
        let mut frame_number = 1;


        loop {
            let frame_path = frames_dir.join(format!("frame_{:06}.png", frame_number));

            if !frame_path.exists() {
                break;
            }
            match image::open(&frame_path) {
                Ok(img) => {
                    let timestamp = frame_number as f64 / fps.unwrap_or(30) as f64;
                    frames.push(VideoFrame::new(img.to_rgb8(), frame_number, timestamp));

                    frame_number += 1;
                }
                Err(_) => break,
            }
        }
        println!("Извлечено {} кадров", frames.len());
        Ok(frames)
    }

    pub fn save_frames_to_video(&self,
                                frames: &[VideoFrame],
                                output_path: &str,
                                fps: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let frames_dir = self.temp_dir.path().join("output_frames");
        fs::create_dir_all(&frames_dir)?;

        //Save all frame like png

        for (i, frame) in frames.iter().enumerate() {
            let frame_path = frames_dir.join(format!("frame_{:06}.png", i + 1));
            frame.data.save(&frame_path)?;
        }
        let frame_pattern = frames_dir.join("frame_%06d.png").to_string_lossy().to_string();

        let status = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-framerate", &fps.to_string(),
                "-i", &frame_pattern,
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                "-crf", "23",
                output_path
            ])
            .status()?;

        if !status.success() {
            return Err("Error from create video".into());
        }

        println!("Video create: {}", output_path);
        Ok(())
    }
    pub fn create_derivative_video(
        &self, input_video_path: &str,
        output_video_path: &str,
        fps: u32,
        threshold: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::processors::VideoDerivativeProcessor;
        use crate::types::ProcessingConfig;

        //Get frame
        let frames = self.extract_frames_from_video(input_video_path, Some(fps))?;

        let config = ProcessingConfig{
            fps, threshold, ..Default::default()
        };
        let mut processor = VideoDerivativeProcessor::new(config);
        let mut derivative_frames = Vec::new();

        for frame in frames{
            let derivative_frame = processor.process_frame(&frame);
            derivative_frames.push(derivative_frame);
        }

        self.save_frames_to_video(&derivative_frames, output_video_path, fps)?;

        Ok(())
    }
}