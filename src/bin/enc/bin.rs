use lib::{
	env::{init, env},
	camera,
	image,
	frame_wave,
	audio,
	misc::throttle,
};
use rscam::Camera;
use std::time::Instant;

fn capture_frame (cam : &Camera, writer : &mut audio::Writer) {
	let frame : rscam::Frame = cam.capture().unwrap();

	// Converts a camera capture into a vector of brightness values between
	// 64 and 255, into 50 rows of 78 u8's
	let pixels = image::convert(&frame);

	// Makes sure each row and end of frame contains a
	// hsync and vsync set of samples respectively
	let output = frame_wave::convert_frame(&pixels);

	for sample in output {
		let _ = writer.write_sample((sample as i16 - 128) * 256);
	}
}

fn main () {
	init();

	let cam = camera::setup();

	// let mut buffer : Vec<u8> = Vec::with_capacity(88200);
	let mut writer = audio::create_writer();

	let fps = env("OUTPUT_FPS").parse::<u64>().unwrap();
	let length = env("RECORDING_LENGTH").parse::<u64>().unwrap();
	let frames = fps * length;

	// Convert frame rate to milliseconds per frame
	let max_duration = 1000.0 / fps as f64;

	let total_start = Instant::now();

	for _ in 0..frames {
		let start = Instant::now();
		capture_frame(&cam, &mut writer);
		throttle(start, max_duration as u64);
	}

	let _ = writer.finalize();

	println!("{:?}", total_start.elapsed());
}
