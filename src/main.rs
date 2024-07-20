use std::io::{self, BufRead, Write};
use image::{RgbImage, Rgb};
use png::Encoder;

fn char_to_color(c: char) -> Rgb<u8> {
    match c {
        '.' => Rgb([0, 0, 0]),     // Black
        '#' => Rgb([255, 255, 255]), // White
        'R' => Rgb([255, 0, 0]),   // Red
        'O' => Rgb([255, 165, 0]), // Orange
        'Y' => Rgb([255, 255, 0]), // Yellow
        'G' => Rgb([0, 128, 0]),   // Green
        'B' => Rgb([0, 0, 255]),   // Blue
        'I' => Rgb([75, 0, 130]),  // Indigo
        'V' => Rgb([238, 130, 238]), // Violet
        _ => panic!("Unsupported color character: {}", c), // Panic on unsupported color
    }
}

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().filter_map(|line| line.ok()).collect();

    let height = input.len();
    let width = if height > 0 { input[0].len() } else { 0 };

    let mut img = RgbImage::new(width as u32, height as u32);

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            img.put_pixel(x as u32, y as u32, char_to_color(c));
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut encoder = Encoder::new(&mut handle, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let data: Vec<u8> = img.into_raw();
    writer.write_image_data(&data).unwrap();
}
