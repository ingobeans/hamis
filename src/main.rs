use std::{env::args, io::Cursor};

use hamis::draw_image;
use image::guess_format;

const HELP: &str = "usage: hamis <image path> [scale factor]";

fn main() {
    let mut args = args();
    let Some(image_path) = args.nth(1) else {
        println!("{HELP}");
        return;
    };
    let scale_factor = match args.next() {
        Some(value) => match value.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("error: invalid scale factor.\n{HELP}");
                return;
            }
        },
        None => 1,
    };

    // read file data to buffer
    let data = match std::fs::read(image_path) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    // guess image format of file specified
    let Ok(format) = guess_format(&data) else {
        println!("couldn't infer format of image");
        return;
    };

    // image::load needs BufRead + Seek, so i wrap file buffer in a cursor
    let cursor = Cursor::new(data);

    let image = image::load(cursor, format).unwrap();

    draw_image(&image, scale_factor);
}
