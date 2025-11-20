pub mod types;
pub mod processors;
pub mod video;
pub mod utils;

// Re-export основных компонентов для удобства использования
pub use types::{VideoFrame, ProcessingConfig};
pub use processors::VideoDerivativeProcessor;
pub use video::VideoProcessor;
pub use utils::FrameGenerator;

#[cfg(test)]
mod tests {

    use image::{ImageBuffer, Rgb};
    use crate::{ProcessingConfig, VideoDerivativeProcessor, VideoFrame};

    #[test]
    fn test_video_frame_creation() {
        let frame_data = ImageBuffer::from_pixel(100, 100, Rgb([255, 0, 0]));
        let frame = VideoFrame::new(frame_data, 1, 0.033);

        assert_eq!(frame.width(), 100);
        assert_eq!(frame.height(), 100);
        assert_eq!(frame.frame_number, 1);
    }

    #[test]
    fn test_processor_creation() {
        let config = ProcessingConfig::default();
        let processor = VideoDerivativeProcessor::new(config);

        // Проверяем, что процессор создан и конфигурация установлена
        assert_eq!(processor.get_config().threshold, 30);
        assert_eq!(processor.get_config().fps, 30);
    }

    #[test]
    fn test_frame_difference() {
        let frame1_data = ImageBuffer::from_pixel(10, 10, Rgb([100, 100, 100]));
        let frame2_data = ImageBuffer::from_pixel(10, 10, Rgb([150, 150, 150]));

        let config = ProcessingConfig::default();
        let mut processor = VideoDerivativeProcessor::new(config);

        let vid_frame1 = VideoFrame::new(frame1_data, 0, 0.0);
        let vid_frame2 = VideoFrame::new(frame2_data, 1, 0.033);

        // Первый кадр должен дать черное изображение
        let derivative1 = processor.process_frame(&vid_frame1);
        assert_eq!(derivative1.data.get_pixel(5, 5)[0], 0);

        // Второй кадр должен показать разницу
        let derivative2 = processor.process_frame(&vid_frame2);
        assert_eq!(derivative2.data.get_pixel(5, 5)[0], 50); // 150 - 100 = 50
    }

    #[test]
    fn test_config_default() {
        let config = ProcessingConfig::default();
        assert_eq!(config.threshold, 30);
        assert_eq!(config.fps, 30);
        assert_eq!(config.output_width, 640);
        assert_eq!(config.output_height, 480);
        assert!(config.noise_reduction);
    }
}