use std::io::{Write, stdout};

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
};
use image::{DynamicImage, GenericImageView};

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

pub fn draw_image(image: &DynamicImage) {
    let row = vec![None; image.width() as usize];
    let mut buf = vec![row; image.height() as usize];
    let pixels = image.pixels();
    for (x, y, color) in pixels {
        buf[y as usize][x as usize] = Some(color);
    }

    let mut stdout = stdout();
    for y in (0..image.height()).step_by(2) {
        for x in 0..image.width() {
            let top_rgba = buf[y as usize][x as usize].unwrap().0;
            let top_color = rgba_to_color(top_rgba);

            let bottom_rgba;

            if y == image.height() {
                // if at last row, pretend bottom pixel is transparent
                bottom_rgba = [0, 0, 0, 0];
            } else {
                // if not at last row, read rgba of below pixel
                bottom_rgba = buf[y as usize + 1][x as usize].unwrap().0;
            }

            let bottom_color = rgba_to_color(bottom_rgba);

            if top_rgba[3] == 0 {
                if bottom_rgba[3] == 0 {
                    // if both pixels are empty
                    queue!(stdout, ResetColor).unwrap();
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
        println!("");
    }
    queue!(stdout, ResetColor).unwrap();
    stdout.flush().unwrap();
}
