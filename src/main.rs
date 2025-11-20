use video_derivative::{
    VideoDerivativeProcessor,
    VideoProcessor,
    FrameGenerator,
    ProcessingConfig,
    VideoFrame
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Video Derivative Processor");

    // Демонстрация с генерацией тестового видео
    // demo_with_generated_frames()?;

    // Раскомментируйте для обработки реального видео
    demo_with_real_video()?;

    Ok(())
}

fn demo_with_generated_frames() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Генерация тестового видео...");

    // Создаем генератор кадров
    let generator = FrameGenerator::new(640, 480, 30);
    let frames = generator.generate_complex_scene_frames(60);

    // Обрабатываем производные
    let config = ProcessingConfig {
        threshold: 25,
        fps: 30,
        ..Default::default()
    };

    let mut processor = VideoDerivativeProcessor::new(config);
    let mut derivative_frames = Vec::new();

    for frame in &frames {
        let derivative_frame = processor.process_frame(frame);
        derivative_frames.push(derivative_frame);
    }

    // Сохраняем результат
    let video_processor = VideoProcessor::new()?;
    video_processor.save_frames_to_video(&derivative_frames, "derivative_output.mp4", 30)?;

    println!("✅ Тестовое видео создано: derivative_output.mp4");
    Ok(())
}

fn demo_with_real_video() -> Result<(), Box<dyn std::error::Error>> {
    let video_path = "input_video.mp4";

    if !std::path::Path::new(video_path).exists() {
        println!("❌ Файл {} не найден", video_path);
        return Ok(());
    }

    println!("🎬 Обработка реального видео...");

    let video_processor = VideoProcessor::new()?;
    video_processor.create_derivative_video(
        video_path,
        "real_video_derivative.mp4",
        30,
        30,
    )?;

    println!("✅ Обработка завершена: real_video_derivative.mp4");
    Ok(())
}
