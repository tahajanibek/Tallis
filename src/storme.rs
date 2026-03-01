use image::{DynamicImage, GenericImageView, imageops::FilterType};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::fs;

pub struct PageNumberExtractor {
    engine: OcrEngine,
}

impl PageNumberExtractor {
    pub fn new(detect_path: &str, rec_path: &str) -> Option<Self> {
        let detect_model = Model::load(fs::read(detect_path).ok()?).ok()?;
        let rec_model = Model::load(fs::read(rec_path).ok()?).ok()?;
        let engine = OcrEngine::new(OcrEngineParams {
            detection_model: Some(detect_model),
            recognition_model: Some(rec_model),
            ..Default::default()
        })
        .ok()?;
        Some(Self { engine })
    }

    pub fn extract(&self, img: &DynamicImage, side: Option<&str>, top: bool) -> Option<String> {
        let (w, h) = img.dimensions();
        let scan_height = (h as f32 * 0.08) as u32;
        let y_start = if top { 0 } else { h.saturating_sub(scan_height) };

        let (x_start, scan_width) = match side {
            Some("left")  => (0, (w as f32 * 0.30) as u32),
            Some("right") => {
                let sw = (w as f32 * 0.30) as u32;
                (w.saturating_sub(sw), sw)
            }
            _ => (0, w),
        };

        let cropped = img.crop_imm(x_start, y_start, scan_width, scan_height);
        let processed = preprocess_for_numbers(&cropped);
        self.run_ocr(&processed)
    }

    fn run_ocr(&self, img: &DynamicImage) -> Option<String> {
        let rgb_img = img.to_rgb8();
        let (cw, ch) = rgb_img.dimensions();
        let img_source = ImageSource::from_bytes(rgb_img.as_raw(), (cw, ch)).ok()?;
        let ocr_input = self.engine.prepare_input(img_source).ok()?;
        let text = self.engine.get_text(&ocr_input).ok()?;

        ///println!("DEBUG OCR:\n{}", text);

       
        if let Some(result) = text.lines().find_map(|line| {
            let trimmed = line.trim();
            if trimmed.chars().all(|c| c.is_ascii_digit())
                && trimmed.len() >= 2
                && trimmed.len() <= 4
                && trimmed != "0"
            {
                Some(trimmed.to_string())
            } else {
                None
            }
        }) {
            return Some(result);
        }

        text.lines().find_map(|line| {
            let trimmed = line.trim();

            if trimmed.split_whitespace().count() > 2 {
                return None;
            }
            let only_digits: String = trimmed
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();
            if only_digits.len() >= 2
                && only_digits.len() <= 4
                && only_digits != "0"
                && !trimmed.chars().any(|c| c.is_alphabetic())
            {
                Some(only_digits)
            } else {
                None
            }
        })
    }
}

fn preprocess_for_numbers(img: &DynamicImage) -> DynamicImage {
    let gray = img.to_luma8();

    let scaled = image::imageops::resize(
        &gray,
        gray.width() * 4,
        gray.height() * 4,
        FilterType::Lanczos3,
    );
    DynamicImage::ImageLuma8(scaled)
}