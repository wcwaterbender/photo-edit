use std::error::Error;
use std::io::prelude::*;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, open};
use show_image::{ImageView, ImageInfo, create_window};

fn file_main() -> Result<(), Box<dyn Error>>{
    //accept input from cmd line for image uri file location
    let mut args = std::env::args().skip(1);
    let file = match args.next() {
        Some(s) => s,
        None => Err("err reading filepath")?
    }; 

    //import test image
    let on_top = open(file).unwrap().into_rgb8();
    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        if (x + y) % 2 == 0 {
            image::Rgb([0, 0, 0])
        } else {
            image::Rgb([255, 255, 255])
        }
    });
    
    let image = ImageView::new(ImageInfo::rgb8(1920, 1080), &img);
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    Ok(())
}
#[show_image::main]
fn main() {
    let result = file_main();
    if let Err(err) = result {
        let _ = writeln!(std::io::stderr(), "{}", err);
    }
}

// #[show_image::main]
// fn main() -> Result<(), Box<dyn std::error::Error>> {

//   let image = ImageView::new(ImageInfo::rgb8(1920, 1080), pixel_data);

//   // Create a window with default options and display the image.
//   let window = create_window("image", Default::default())?;
//   window.set_image("image-001", image)?;

//   Ok(())
// }