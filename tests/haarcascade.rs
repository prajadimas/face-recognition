
use std::{error::Error, path::PathBuf};

use opencv::{
	core,
    imgcodecs,
	objdetect,
	prelude::*,
	Result,
	types,
};

use structopt::StructOpt;

use image::{Rgba};
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

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let input_img = image::open(&opt.input)?;

    #[cfg(ocvrs_opencv_branch_32)]
	let (xml, input_image) = {
		(
			"/usr/share/OpenCV/haarcascades/haarcascade_frontalface_alt.xml".to_owned(),
			imgcodecs::imread(&opt.input.into_os_string().into_string().unwrap(), 0)?,
		)
	};
	#[cfg(not(ocvrs_opencv_branch_32))]
	let (xml, input_image) = {
		(
			core::find_file("haarcascades/haarcascade_frontalface_alt.xml", true, false)?,
			imgcodecs::imread(&opt.input.into_os_string().into_string().unwrap(), 0)?,
		)
	};

    let mut face = objdetect::CascadeClassifier::new(&xml)?;

    /* let mut gray = Mat::default();

    imgproc::cvt_color(
        &input_image,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
    )?;

    let mut reduced = Mat::default();

    imgproc::resize(
        &gray,
        &mut reduced,
        core::Size {
            width: 0,
            height: 0,
        },
        0.25f64,
        0.25f64,
        imgproc::INTER_LINEAR,
    )?; */

    let mut faces = types::VectorOfRect::new();

    face.detect_multi_scale(
        &input_image,
        &mut faces,
        1.1,
        3,
        objdetect::CASCADE_SCALE_IMAGE,
        core::Size {
            width: 20,
            height: 20,
        },
        core::Size {
            width: 0,
            height: 0,
        },
    )?;

    println!("Faces: {}", faces.len());

    // We want to change input_image since it is not needed.
    let mut output_image = input_img;

    for face in faces {
        println!("Face {:?}", face);
        let rect = Rect::at((face.x) as i32, (face.y) as i32)
            .of_size((face.width) as u32, (face.height) as u32);

        // Draw a green line around the bounding box
        draw_hollow_rect_mut(&mut output_image, rect, LINE_COLOUR);
    }

    // Once we've modified the image we save it in the output location.
    output_image.save(&opt.output)?;

    Ok(())
}
