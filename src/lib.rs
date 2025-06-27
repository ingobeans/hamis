use std::io::{stdout, Write};

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
};
use image::{DynamicImage, GenericImageView};

/// Convert rgba value \[u8;4\] to a [crossterm color](crossterm::style::Color)
fn rgba_to_color(rgba: [u8; 4]) -> crossterm::style::Color {
    match rgba[3] {
        0 => crossterm::style::Color::Reset,
        _ => crossterm::style::Color::Rgb {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
        },
    }
}

/// Print [DynamicImage] to stdout
pub fn draw_image(image: &DynamicImage, scale_factor: u32) {
    let mut stdout = stdout();

    // iterate over all pixels (y,x).

    // we print 2 pixels at a time, since each character in a terminal's height is roughly double its width,
    // meaning each character can represent two pixels.
    // therefore we step y by two, and print the above and below pixel
    for y in (0..image.height() * scale_factor).step_by(2) {
        for x in 0..image.width() * scale_factor {
            let top_rgba = image.get_pixel(x / scale_factor, y / scale_factor).0;
            let top_color = rgba_to_color(top_rgba);

            let bottom_rgba = if y / scale_factor == image.height() {
                // if at last row, pretend bottom pixel is transparent
                [0, 0, 0, 0]
            } else {
                // if not at last row, read rgba of below pixel
                image.get_pixel(x / scale_factor, (y + 1) / scale_factor).0
            };

            let bottom_color = rgba_to_color(bottom_rgba);

            if top_rgba[3] == 0 {
                queue!(stdout, ResetColor).unwrap();
                if bottom_rgba[3] == 0 {
                    // if both pixels are empty
                    print!(" ");
                    continue;
                }
                // if only top pixel is empty
                queue!(stdout, SetForegroundColor(bottom_color)).unwrap();
                print!("▄");
                continue;
            }

            // if neither pixel is empty
            queue!(stdout, SetBackgroundColor(bottom_color)).unwrap();
            queue!(stdout, SetForegroundColor(top_color)).unwrap();

            print!("▀");
        }
        queue!(stdout, ResetColor).unwrap();
        println!();
    }
    stdout.flush().unwrap();
}
