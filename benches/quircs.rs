use std::io::Cursor;
use std::path::Path;
use criterion::{Criterion, criterion_group, criterion_main};
use image::{ImageBuffer, Rgb, RgbImage};
use fqrs::decode_qr;

pub fn encode_jpeg(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, quality: u8) -> Option<Vec<u8>> {
    let mut jpeg_data = Vec::new();
    let mut cursor = Cursor::new(&mut jpeg_data);

    match img.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => Some(jpeg_data),
        Err(_) => None,
    }
}

pub fn byte_array_to_jpeg(byte_array: &[u8], width: u32, height: u32, quality: u8) -> Option<Vec<u8>> {
    // Convert byte array to ImageBuffer
    let img: RgbImage = match ImageBuffer::from_raw(width, height, byte_array.to_vec()) {
        Some(img) => img,
        None => return None,
    };

    // Encode to JPEG
    let mut jpeg_data = Vec::new();
    let mut cursor = Cursor::new(&mut jpeg_data);

    match img.write_to(&mut cursor, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => Some(jpeg_data),
        Err(_) => None,
    }
}

// [5.9509 ms 5.9560 ms 5.9617 ms]
fn criterion_benchmark(c: &mut Criterion) {
    let w = 532;
    let h = 768;

    let file_path = Path::new("./qr.jpg");

    // Open the image
    let img = image::open(file_path).unwrap();

    let img_bytes = img.as_bytes();

    let img_jpeg  = byte_array_to_jpeg(img_bytes, w.clone(), h.clone(), 100).unwrap();

    c.bench_function("quircs decode qr", |b| b.iter(|| {
        decode_qr(w as usize, h as usize, &img_jpeg).unwrap_or("".to_string());
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);