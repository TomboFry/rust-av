use rscam::Frame;
use crate::env::env;
// use crate::misc::number_to_digits;

pub fn convert (frame : &Frame) -> Vec<Vec<u8>> {
	let width_output = env("OUTPUT_WIDTH")
		.parse::<u32>()
		.expect("OUTPUT_WIDTH is not an integer");

	let height_output = env("OUTPUT_HEIGHT")
		.parse::<u32>()
		.expect("OUTPUT_HEIGHT is not an integer");

	let img = image::load_from_memory(&frame[..]).expect("Could not load camera image");

	// Convert into small, greyscale image
	let img = img.resize_exact(width_output, height_output, image::imageops::Nearest);
	let img = img.grayscale();

	// let _ = img.save(&format!("frame-{}.png", number_to_digits(index, 4)));

	// Scale the captured values to 64-255
	let img : Vec<u8> = img
		.raw_pixels()
		.into_iter()
		.map(|x : u8| {
			(x as f64 * 0.75) as u8 + 64
		})
		.collect();
	
	// Split them into chunks of our output width
	img
		.chunks_exact(width_output as usize)
		.map(|x| x.to_vec())
		.collect()
}
