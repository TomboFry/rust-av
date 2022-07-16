use rscam::{Camera};
use crate::env::env;

pub fn setup () -> Camera {
	let mut camera = rscam::new(&env("VIDEO_INPUT"))
		.expect("Could not load camera");

	let width = env("INPUT_WIDTH")
		.parse::<u32>()
		.expect("INPUT_WIDTH is not an integer");

	let height = env("INPUT_HEIGHT")
		.parse::<u32>()
		.expect("INPUT_HEIGHT is not an integer");

	let fps = env("INPUT_FPS")
		.parse::<u32>()
		.expect("INPUT_FPS is not an integer");

	camera.start(&rscam::Config {
		interval: (1, fps),
		resolution: (width, height),
		format: b"MJPG",
		..Default::default()
	}).unwrap();

	camera
}
