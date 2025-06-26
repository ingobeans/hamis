use std::{env::args, io::Cursor};

use hamis::draw_image;
use image::guess_format;

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

    draw_image(&image);
}
