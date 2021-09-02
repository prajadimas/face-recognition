
use std::{error::Error, path::PathBuf};

use rustface::{ImageData};

use structopt::StructOpt;

use image::{Rgba, DynamicImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

#[derive(Copy, Clone, Debug)]
// Make it a bit nicer to work with the results, by adding a more explanatory struct
pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub prob: f32,
}

// The line colour never changes, so make it a `const`
const LINE_COLOUR: Rgba<u8> = Rgba([0, 255, 0, 0]);

fn get_test_image(img: &PathBuf) -> DynamicImage {
	// println!("{:?}", img);
    let image: DynamicImage = match image::open(img) {
        Ok(image) => image,
        Err(message) => {
            println!("Failed to read image: {}", message);
            std::process::exit(1);
        }
    };

    return image;
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

	let mut detector = rustface::create_detector("./models/seeta_fd_frontal_v1.0.bin").unwrap();
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

	let input_img = get_test_image(&opt.input).to_luma();
	let input_image = image::open(&opt.input)?;

    // convert to rustface internal image datastructure
    let (width, height) = input_img.dimensions();

	// println!("{:?}x{:?}", width, height);

	let mut image = ImageData::new(&input_img, width, height);

	// We want to change input_image since it is not needed.
	let mut output_image = input_image;

    for face in detector.detect(&mut image).into_iter() {
        // print confidence score and coordinates
        println!("Found face: {:?}", face);

		let rect = Rect::at((face.bbox().x()) as i32, (face.bbox().y()) as i32)
            .of_size((face.bbox().width()) as u32, (face.bbox().height()) as u32);

        // Draw a green line around the bounding box
        draw_hollow_rect_mut(&mut output_image, rect, LINE_COLOUR);

    }

    // Once we've modified the image we save it in the output location.
    output_image.save(&opt.output)?;

    Ok(())
}
