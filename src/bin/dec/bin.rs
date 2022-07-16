use std::env::args;
use std::path::Path;
// use lib::env::{init, env};
use image::{ImageBuffer, GrayImage, imageops};

enum SampleType {
	Hsync,
	Vsync,
	Pixel(u8)
}

fn parse_sample (sample : i16) -> SampleType {
	if sample >= i16::MIN && sample < i16::MIN + 8192 {
		return SampleType::Vsync;
	}
	if sample >= i16::MIN + 8192 && sample < i16::MIN + 16384 {
		return SampleType::Hsync;
	}

	let sample_u8 = ((sample as i32 + 32768) / 256) as u8;
	SampleType::Pixel(sample_u8)
}

fn read_wav (path : &str) -> Vec<i16> {
	let mut reader = hound::WavReader::open(path).unwrap();
	let sqr_sum = reader.samples::<i16>().map(|s| s.unwrap()).collect();
	sqr_sum
}

fn detect_frame_specs (samples: &[SampleType], file: &str) {
	let mut last_hsync_len = 0;
	let mut last_vsync_len = 0;
	let mut frame_current = 0;

	let mut line = vec![];
	let mut frame: Vec<Vec<u8>> = vec![];

	for sample in samples {
		match sample {
			SampleType::Hsync => {
				last_hsync_len += 1;
			},
			SampleType::Vsync => {
				last_vsync_len += 1;
			},
			SampleType::Pixel(pixel) => {
				// New Line
				if last_hsync_len > 0 {
					last_hsync_len = 0;
					frame.push(line);
					line = vec![];
				}

				// New Frame
				if last_vsync_len > 0 {
					last_hsync_len = 0;
					last_vsync_len = 0;
					
					let mut image: GrayImage = ImageBuffer::new(
						frame[0].len() as u32,
						frame.len() as u32
					);

					for (line_index, line) in frame.iter().enumerate() {
						for (pixel_index, pixel) in line.iter().enumerate() {
							image.put_pixel(
								pixel_index as u32,
								line_index as u32,
								image::Luma([*pixel])
							);
						}
					}

					let image = imageops::resize(
						&image,
						image.width() * 4,
						image.height() * 4,
						imageops::FilterType::Nearest
					);

					image
						.save(&format!("{}-{}.png", file, frame_current))
						.unwrap();
					
					frame_current += 1;
					frame = vec![];
				}

				line.push(*pixel);
			},
		};
	}
}

pub fn main () {
	// Get list of filenames to parse
	let arguments = args().skip(1).collect::<Vec<_>>();
	let mut files_valid : Vec<String> = Vec::with_capacity(arguments.len());

	// Only use filenames that exist.
	for arg in arguments {
		if Path::new(&arg).exists() {
			files_valid.push(arg);
		} else {
			eprintln!("Skipping file `{}`, does not exist", arg);
		}
	}

	for file in files_valid {
		let samples = read_wav(&file);
		let parsed_samples = samples
			.into_iter()
			.map(|sample| parse_sample(sample))
			.collect::<Vec<_>>();

		detect_frame_specs(&parsed_samples, &file);
	}
}
