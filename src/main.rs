use std::{
    env::args,
    io::{Cursor, Write, stdout},
};

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
};
use image::{GenericImageView, guess_format};

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
            let top_pixel = buf[y as usize][x as usize].unwrap().0;
            let top_color = crossterm::style::Color::Rgb {
                r: top_pixel[0],
                g: top_pixel[1],
                b: top_pixel[2],
            };

            let bottom_pixel = buf[y as usize + 1][x as usize].unwrap().0;
            let bottom_color = crossterm::style::Color::Rgb {
                r: bottom_pixel[0],
                g: bottom_pixel[1],
                b: bottom_pixel[2],
            };
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
