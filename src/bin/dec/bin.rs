use image::{imageops, GrayImage, ImageBuffer};
use std::env::args;
use std::path::Path;

fn read_wav(path: &Path) -> Vec<i16> {
	let mut reader = hound::WavReader::open(path).unwrap();
	let sqr_sum = reader.samples::<i16>().map(|s| s.unwrap()).collect();
	sqr_sum
}

fn get_frame_width(samples: &[Vec<u8>]) -> usize {
	samples
		.iter()
		.max_by(|x, y| x.len().cmp(&y.len()))
		.unwrap()
		.len()
}

fn parse_frames(samples: &[i16], path: &Path) {
	let mut last_hsync_len = 0;
	let mut last_vsync_len = 0;
	let mut frame_current = 0;

	let mut line = vec![];
	let mut frame = vec![];

	let input_filename = path.file_stem().to_owned().unwrap().to_str().unwrap();

	for sample in samples {
		let smpu8 = ((*sample as i32 + i16::MAX as i32) / 256) as u8;
		if smpu8 < 24 {
			last_vsync_len += 1;
			continue;
		}

		// Basic error correction, if the sync pulse is ambiguous
		if smpu8 >= 24 && smpu8 < 48 {
			if last_hsync_len > 0 {
				last_hsync_len += 1;
				continue;
			}

			last_vsync_len += 1;
			continue;
		}

		if smpu8 >= 48 && smpu8 < 96 {
			last_hsync_len += 1;
			continue;
		}

		// New Line
		if last_hsync_len > 0 {
			last_hsync_len = 0;
			frame.push(line);
			line = vec![];
		}

		// New Frame - Save Previous
		if last_vsync_len > 0 {
			last_hsync_len = 0;
			last_vsync_len = 0;

			let mut image: GrayImage = ImageBuffer::new(
				get_frame_width(&frame) as u32,
				frame.len() as u32,
			);

			for (line_index, line) in frame.iter().enumerate() {
				for (pixel_index, pixel) in line.iter().enumerate() {
					image.put_pixel(
						pixel_index as u32,
						line_index as u32,
						image::Luma([*pixel]),
					);
				}
			}

			let image = imageops::resize(
				&image,
				image.width() * 4,
				image.height() * 4,
				imageops::FilterType::Nearest,
			);

			image
				.save(&format!("{}-{}.png", input_filename, frame_current))
				.unwrap();

			frame_current += 1;
			frame = vec![];
		}

		let pixel = (*sample / 128).clamp(0, 255) as u8;
		line.push(pixel);
	}
}

pub fn main() {
	// Get list of filenames to parse
	let arguments = args().skip(1).collect::<Vec<_>>();
	let mut files_valid: Vec<String> = Vec::with_capacity(arguments.len());

	if arguments.len() == 0 {
		println!(
			"Decode TVA audio into image files
(you'll have to do the video conversion, unfortunately.)

Usage:
  tva-dec <filenames>..."
		);
		return;
	}

	// Only use filenames that exist.
	for arg in arguments {
		if Path::new(&arg).exists() {
			files_valid.push(arg);
		} else {
			eprintln!("Skipping `{}`, file does not exist", arg);
		}
	}

	for file in files_valid {
		let path = Path::new(&file);
		let samples = read_wav(&path);
		parse_frames(&samples, &path);
	}
}
