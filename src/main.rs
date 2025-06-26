use std::{env::args, io::Cursor};

use hamis::draw_image;
use image::guess_format;

fn main() {
    let mut args = args();
    let Some(image_path) = args.nth(1) else {
        println!("usage: hamis <image path>");
        return;
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

    draw_image(&image);
}
