use std::{
    env::args,
    io::{Cursor, Write, stdout},
};

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
};
use image::{GenericImageView, guess_format};

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

fn main() {
    let mut args = args();
    let Some(image_path) = args.nth(1) else {
        println!("specify image path");
        return;
    };
    let data = std::fs::read(image_path).unwrap();
    let format = guess_format(&data).unwrap();

    let cursor = Cursor::new(data);
    let image = image::load(cursor, format).unwrap();

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

            let bottom_rgba = buf[y as usize + 1][x as usize].unwrap().0;
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
