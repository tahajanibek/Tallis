use image::GenericImageView;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin test_ocr -- <image.jpg>");
        return;
    }

    let path = &args[1];
    let img = image::open(path).expect("Resim açılamadı");
    let (w, h) = img.dimensions();
    println!("Resim boyutu: {}x{}", w, h);

    // Alt %10'u kırp
    let scan_height = (h as f32 * 0.08) as u32;
    let y_start = h.saturating_sub(scan_height);
    let cropped = img.crop_imm(0, y_start, w, scan_height);
    cropped.save("debug_cropped.jpg").unwrap();
    println!("Kırpılmış bölge kaydedildi: debug_cropped.jpg ({}x{})", w, scan_height);

    // Threshold olmadan sadece grayscale + 2x büyüt
    let gray = cropped.to_luma8();
    let scaled = image::imageops::resize(
        &gray,
        gray.width() * 2,
        gray.height() * 2,
        image::imageops::FilterType::Lanczos3,
    );
    let processed = image::DynamicImage::ImageLuma8(scaled);
    processed.save("debug_processed.jpg").unwrap();
    println!("İşlenmiş görüntü kaydedildi: debug_processed.jpg");

    // OCR
    let detect_model = Model::load(fs::read("core/text-detector.onnx").unwrap()).unwrap();
    let rec_model = Model::load(fs::read("core/text-recognitor.onnx").unwrap()).unwrap();
    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detect_model),
        recognition_model: Some(rec_model),
        ..Default::default()
    })
    .unwrap();

    let rgb = processed.to_rgb8();
    let (cw, ch) = rgb.dimensions();
    let source = ImageSource::from_bytes(rgb.as_raw(), (cw, ch)).unwrap();
    let input = engine.prepare_input(source).unwrap();
    let text = engine.get_text(&input).unwrap();

    println!("\n=== OCR ÇIKTISI ===\n{}\n==================", text);
}
