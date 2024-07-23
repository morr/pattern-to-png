use image::{Rgb, RgbImage};
use png::Encoder;
use std::env;
use std::io::{self, BufRead};

fn char_to_color(c: char) -> Rgb<u8> {
    match c {
        'B' => Rgb([0, 0, 0]),                             // Black
        'I' => Rgb([29, 43, 83]),                          // Indigo
        'P' => Rgb([126, 37, 83]),                         // Purple
        'E' => Rgb([0, 135, 81]),                          // Emerald
        'N' => Rgb([171, 82, 54]),                         // browN
        'D' => Rgb([95, 87, 79]),                          // Dead, Dark
        'A' => Rgb([194, 195, 199]),                       // Alive, grAy
        'W' => Rgb([255, 241, 232]),                       // White
        'R' => Rgb([255, 0, 77]),                          // Red
        'O' => Rgb([255, 163, 0]),                         // Orange
        'Y' => Rgb([255, 236, 39]),                        // Yellow
        'G' => Rgb([0, 228, 54]),                          // Green
        'U' => Rgb([41, 173, 255]),                        // blUe
        'S' => Rgb([131, 118, 156]),                       // Slate
        'K' => Rgb([255, 119, 168]),                       // pinK
        'F' => Rgb([255, 204, 170]),                       // Fawn
        'b' => Rgb([41, 24, 20]),                          // black
        'i' => Rgb([17, 29, 53]),                          // indigo
        'p' => Rgb([66, 33, 54]),                          // purple
        'e' => Rgb([18, 83, 89]),                          // emerald
        'n' => Rgb([116, 47, 41]),                         // brown
        'd' => Rgb([73, 51, 59]),                          // dead, dark
        'a' => Rgb([162, 136, 121]),                       // alive, gray
        'w' => Rgb([243, 239, 125]),                       // white
        'r' => Rgb([190, 18, 80]),                         // red
        'o' => Rgb([255, 108, 36]),                        // orange
        'y' => Rgb([168, 231, 46]),                        // yellow
        'g' => Rgb([0, 181, 67]),                          // green
        'u' => Rgb([6, 90, 181]),                          // blue
        's' => Rgb([117, 70, 101]),                        // slate
        'k' => Rgb([255, 110, 89]),                        // pink
        'f' => Rgb([255, 157, 129]),                       // fawn
        'C' => Rgb([0, 255, 255]),                         // Cyan
        'c' => Rgb([95, 205, 228]),                        // cyan
        'H' => Rgb([228, 187, 64]),                        // Honey
        'h' => Rgb([138, 111, 48]),                        // honey
        'J' => Rgb([75, 105, 47]),                         // Jungle
        'j' => Rgb([69, 16, 126]),                         // jungle
        'L' => Rgb([132, 126, 135]),                       // Light
        'l' => Rgb([105, 106, 106]),                       // light
        'M' => Rgb([255, 0, 255]),                         // Magenta
        'm' => Rgb([156, 9, 204]),                         // magenta
        'Q' => Rgb([155, 173, 183]),                       // aQua
        'q' => Rgb([63, 63, 116]),                         // aqua
        'T' => Rgb([55, 148, 110]),                        // Teal
        't' => Rgb([50, 60, 57]),                          // teal
        'V' => Rgb([143, 151, 74]),                        // oliVe
        'v' => Rgb([82, 75, 36]),                          // olive
        'X' => Rgb([255, 0, 0]),                           // X
        'x' => Rgb([217, 87, 99]),                         // x
        'Z' => Rgb([255, 255, 255]),                       // Z
        'z' => Rgb([203, 219, 252]),                       // z
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
