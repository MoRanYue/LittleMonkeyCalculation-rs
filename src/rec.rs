use std::error::Error;

use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use ocrs::{ImageSource, OcrEngine, OcrInput, OcrEngineParams};
use image::DynamicImage;

pub fn create_engine(det_model: Model, rec_model: Model) -> Result<OcrEngine, Box<dyn Error>> {
    Ok(OcrEngine::new(OcrEngineParams {
        detection_model: Some(det_model),
        recognition_model: Some(rec_model),
        ..Default::default()
    })?)
}

pub fn prepare_img(engine: &OcrEngine, pic: DynamicImage) -> Result<OcrInput, Box<dyn Error>> {
    let img = pic.into_rgb8();
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    // convert to greyscale, map range to [-0.5, 0.5].
    Ok(engine.prepare_input(img_source)?)
}

pub fn recognize(engine: &OcrEngine, pic: DynamicImage) -> Result<(), Box<dyn Error>> {
    let ocr_input = prepare_img(engine, pic)?;

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // 在输入图像中基于包围盒检测文本
    let word_rects = engine.detect_words(&ocr_input)?;

    // 将文字按行分组
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // 在每行上识别文本
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    for line in line_texts.iter().flatten().filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }

    Ok(())
}

pub fn recognize_only_text(engine: &OcrEngine, pic: DynamicImage) -> Result<String, Box<dyn Error>> {
    let img = prepare_img(engine, pic)?;
    Ok(engine.get_text(&img)?)
}