use video_derivative::VideoProcessor;
use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Использование: {} <путь_к_видео>", args[0]);
        eprintln!("Пример: {} video.mov", args[0]);
        process::exit(1);
    }

    let video_path = &args[1];


    if let Err(e) = process_video(video_path) {
        eprintln!("Ошибка обработки видео: {}", e);
        process::exit(1);
    }
}

fn process_video(video_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Обработка видео: {}", video_path);


    if !std::path::Path::new(video_path).exists() {
        return Err(format!("Файл не найден: {}", video_path).into());
    }


    let video_processor = VideoProcessor::new()?;

    // Проверяем доступность FFmpeg
    if !video_processor.is_ffmpeg_available() {
        return Err("FFmpeg не найден. Установите FFmpeg для обработки видео.".into());
    }



    // Создаем имя для выходного файла
    let input_path = std::path::Path::new(video_path);
    let output_filename = format!(
        "{}_derivative.mp4",
        input_path.file_stem().unwrap().to_string_lossy()
    );

    println!("🎬 Создание производного видео...");

    // Обрабатываем видео
    video_processor.create_simple_derivative_video(video_path, &output_filename)?;

    println!("Готово! Результат: {}", output_filename);

    Ok(())
}