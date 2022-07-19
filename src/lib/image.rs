pub fn generate_ppm_header(width: u32, height: u32) -> Vec<u8> {
	let ppm_frame = format!("P6\n{} {}\n255\n", width, height)
		.as_bytes()
		.to_owned();

	ppm_frame
}

pub fn generate_raw_pixels(
	frame: &[u8],
	header: &[u8],
	nwidth: u32,
	nheight: u32,
) -> Vec<Vec<u8>> {
	let mut ppm_frame = header.to_owned();

	// Triple each byte
	let frame: Vec<u8> = frame.iter().fold(vec![], |mut x, y| {
		x.extend_from_slice(&vec![*y, *y, *y]);
		x
	});
	ppm_frame.extend_from_slice(&frame);

	let img = image::load_from_memory(&ppm_frame)
		.expect("Could not load image");

	// // Convert into small, greyscale image
	let img = img.resize_exact(nwidth, nheight, image::imageops::Nearest);
	let img = img.grayscale();

	// Scale the captured values to 128-255
	let img: Vec<u8> = img
		.raw_pixels()
		.into_iter()
		.map(|x: u8| (x as f64 * 0.5) as u8 + 128)
		.collect();

	// Split them into chunks of our output width
	img.chunks_exact(nwidth as usize)
		.map(|x| x.to_vec())
		.collect()
}
