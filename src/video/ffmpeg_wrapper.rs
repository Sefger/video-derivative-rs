use crate::types::VideoFrame;
use std::process::{Command, Stdio};
use std::fs;
use tempfile::TempDir;

pub struct VideoProcessor {
    temp_dir: TempDir,
    ffmpeg_available: bool,
}

impl VideoProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let ffmpeg_available = Self::check_ffmpeg_availability();

        Ok(Self {
            temp_dir,
            ffmpeg_available,
        })
    }

    fn check_ffmpeg_availability() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    pub fn is_ffmpeg_available(&self) -> bool {
        self.ffmpeg_available
    }

    pub fn extract_frames_from_video(
        &self,
        video_path: &str,
        fps: Option<u32>,
    ) -> Result<Vec<VideoFrame>, Box<dyn std::error::Error>> {
        if !self.ffmpeg_available {
            return Err("FFmpeg не доступен. Установите FFmpeg для извлечения кадров из видео.".into());
        }

        let frames_dir = self.temp_dir.path().join("extracted_frames");
        fs::create_dir_all(&frames_dir)?;

        let output_pattern = frames_dir.join("frame_%06d.png").to_string_lossy().to_string();

        // Упрощенная команда FFmpeg без конфликтующих аргументов
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(video_path)
            .arg("-qscale:v")  // Качество изображения
            .arg("2")          // Высокое качество
            .arg(&output_pattern)
            .status()?;

        if !status.success() {
            return Err("Ошибка при извлечении кадров из видео".into());
        }

        // Даем время на запись файлов
        std::thread::sleep(std::time::Duration::from_secs(2));

        let mut frames = Vec::new();

        // Ищем все PNG файлы в директории
        let entries: Result<Vec<_>, _> = fs::read_dir(&frames_dir)?.collect();
        let entries = entries?;

        let mut png_files: Vec<_> = entries
            .into_iter()
            .filter(|entry| {
                if let Ok(metadata) = entry.metadata() {
                    metadata.is_file() &&
                        entry.path().extension().is_some_and(|ext| ext == "png")
                } else {
                    false
                }
            })
            .collect();

        // Сортируем файлы по имени
        png_files.sort_by_key(|e| e.path());

        for (i, entry) in png_files.into_iter().enumerate() {
            match image::open(entry.path()) {
                Ok(img) => {
                    let timestamp = i as f64 / fps.unwrap_or(30) as f64;
                    frames.push(VideoFrame::new(img.to_rgb8(), i + 1, timestamp));
                }
                Err(e) => {
                    eprintln!("Ошибка загрузки кадра {}: {}", entry.path().display(), e);
                }
            }
        }

        if frames.is_empty() {
            return Err("Не удалось извлечь ни одного кадра из видео".into());
        }

        println!("Извлечено {} кадров", frames.len());
        Ok(frames)
    }

    pub fn save_frames_to_video(
        &self,
        frames: &[VideoFrame],
        output_path: &str,
        fps: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.ffmpeg_available {
            return Err("FFmpeg не доступен".into());
        }

        let frames_dir = self.temp_dir.path().join("output_frames");
        fs::create_dir_all(&frames_dir)?;

        // Сохраняем все кадры как PNG
        for (i, frame) in frames.iter().enumerate() {
            let frame_path = frames_dir.join(format!("frame_{:06}.png", i + 1));
            frame.save_to_file(&frame_path)?;
        }

        // Создаем видео с помощью ffmpeg
        let frame_pattern = frames_dir.join("frame_%06d.png").to_string_lossy().to_string();

        let status = Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate", &fps.to_string(),
                "-i", &frame_pattern,
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                "-crf", "23",
                output_path,
            ])
            .status()?;

        if !status.success() {
            return Err("Ошибка при создании видео".into());
        }

        println!("Видео успешно создано: {}", output_path);
        Ok(())
    }

    pub fn create_simple_derivative_video(
        &self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.ffmpeg_available {
            return Err("FFmpeg не доступен".into());
        }

        println!("Обработка видео: {} -> {}", input_path, output_path);

        // Используем наш Rust-процессор вместо FFmpeg фильтра
        self.create_derivative_video(input_path, output_path, 30, 25)
    }

    pub fn create_derivative_video(
        &self,
        input_video_path: &str,
        output_video_path: &str,
        fps: u32,
        threshold: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use crate::processors::VideoDerivativeProcessor;
        use crate::types::ProcessingConfig;

        println!("Извлечение кадров из видео...");

        // Извлекаем кадры
        let frames = self.extract_frames_from_video(input_video_path, Some(fps))?;

        if frames.is_empty() {
            return Err("Не удалось извлечь кадры из видео".into());
        }



        let config = ProcessingConfig {
            fps,
            threshold,
            output_width: frames[0].width(),
            output_height: frames[0].height(),
            noise_reduction: true,
        };

        let mut processor = VideoDerivativeProcessor::new(config);
        let mut derivative_frames = Vec::new();

        // Обрабатываем каждый кадр
        for (i, frame) in frames.iter().enumerate() {
            println!("Обработан кадр {}/{}", i + 1, frames.len());

            let derivative_frame = processor.process_frame(frame);
            derivative_frames.push(derivative_frame);
        }

        println!("Создание выходного видео...");
        self.save_frames_to_video(&derivative_frames, output_video_path, fps)?;

        Ok(())
    }
}