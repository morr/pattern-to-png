use image::{Rgb, RgbImage};
use png::Encoder;
use std::env;
use std::io::{self, BufRead};

fn char_to_color(c: char) -> Rgb<u8> {
    match c {
        '.' => Rgb([0, 0, 0]),                             // Black
        '#' => Rgb([255, 255, 255]),                       // White
        'R' => Rgb([255, 0, 0]),                           // Red
        'O' => Rgb([255, 165, 0]),                         // Orange
        'Y' => Rgb([255, 255, 0]),                         // Yellow
        'G' => Rgb([0, 128, 0]),                           // Green
        'B' => Rgb([0, 0, 255]),                           // Blue
        'I' => Rgb([75, 0, 130]),                          // Indigo
        'V' => Rgb([238, 130, 238]),                       // Violet
        _ => panic!("Unsupported color character: {}", c), // Panic on unsupported color
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <magnification factor>", args[0]);
        std::process::exit(1);
    }

    let magnification_arg = &args[1];
    if !magnification_arg.ends_with('x') {
        eprintln!("Magnification factor must be in the format <number>x, e.g., 10x");
        std::process::exit(1);
    }

    let magnification: usize = magnification_arg[..magnification_arg.len() - 1]
        .parse()
        .expect("Magnification factor must be a positive integer");

    let stdin = io::stdin();
    let input: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.starts_with(' '))
        .collect();

    let height = input.len();
    let width = if height > 0 { input[0].len() } else { 0 };

    // Create an image magnified by the specified factor
    let mut img = RgbImage::new(
        (width * magnification) as u32,
        (height * magnification) as u32,
    );

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let color = char_to_color(c);
            // Fill magnification x magnification block with the same color
            for dy in 0..magnification {
                for dx in 0..magnification {
                    img.put_pixel(
                        (x * magnification + dx) as u32,
                        (y * magnification + dy) as u32,
                        color,
                    );
                }
            }
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut encoder = Encoder::new(
        &mut handle,
        (width * magnification) as u32,
        (height * magnification) as u32,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let data: Vec<u8> = img.into_raw();
    writer.write_image_data(&data).unwrap();
}
