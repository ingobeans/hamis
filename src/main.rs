use std::{
    env::args,
    io::{Cursor, Write, stdout},
};

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor},
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
    for y in (0..image.height()).step_by(1) {
        for x in 0..image.width() {
            let pixel = buf[y as usize][x as usize].unwrap();
            let rgb = pixel.0;
            let color = crossterm::style::Color::Rgb {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            };
            queue!(stdout, SetBackgroundColor(color)).unwrap();
            print!(" ");
        }
        queue!(stdout, ResetColor).unwrap();
        println!("");
    }
    queue!(stdout, ResetColor).unwrap();
    stdout.flush().unwrap();
}
