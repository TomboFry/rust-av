use std::env::args;
use std::path::Path;
use lib::env::{init, env};

enum SampleType {
	Hsync,
	Vsync,
	Pixel(u8)
}

fn parse_sample (sample : i16) -> SampleType {
	let sample_u8 = ((sample as i32 + 32768) / 256) as u8;
	println!("{} -> {}", sample, sample_u8);
	SampleType::Pixel(sample_u8)
}

fn read_wav (path : &str) -> Vec<i16> {
	let mut reader = hound::WavReader::open(path).unwrap();
	let sqr_sum = reader.samples::<i16>().map(|s| s.unwrap()).collect();
	sqr_sum
}

pub fn main () {
	init();

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
		for sample in samples { parse_sample(sample); }
	}
}
