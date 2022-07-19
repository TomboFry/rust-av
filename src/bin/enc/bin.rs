extern crate ffmpeg_next as ffmpeg;

use std::{env::args, path::Path};

use ffmpeg::{
	format::{input, Pixel},
	media::Type,
	software::scaling::{context::Context, flag::Flags},
	util::frame::video::Video,
};

use lib::{
	audio,
	env::{env, init},
};

fn main() -> Result<(), ffmpeg::Error> {
	init();

	ffmpeg::init().unwrap();

	// let _fps = env("OUTPUT_FPS").parse::<u64>().unwrap();
	let width_output = env("OUTPUT_WIDTH")
		.parse::<u32>()
		.expect("OUTPUT_WIDTH is not an integer");

	let height_output = env("OUTPUT_HEIGHT")
		.parse::<u32>()
		.expect("OUTPUT_HEIGHT is not an integer");

	// Get list of arguments
	let arguments = args().skip(1).collect::<Vec<_>>();

	for argument in arguments {
		let path = Path::new(&argument);
		let mut writer = audio::create_writer(path);

		if let Ok(mut input_context) = input(&path) {
			println!("Processing '{}'", argument);

			let input = input_context
				.streams()
				.best(Type::Video)
				.ok_or(ffmpeg::Error::StreamNotFound)?;

			let video_stream_index = input.index();

			let mut decoder = input.codec().decoder().video()?;

			let mut scaler = Context::get(
				decoder.format(),
				decoder.width(),
				decoder.height(),
				Pixel::GRAY8,
				width_output,
				height_output,
				Flags::BILINEAR,
			)?;

			let mut frame_index = 0;

			let mut receive_and_process_decoded_frames =
				|decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
					let mut decoded = Video::empty();
					while decoder.receive_frame(&mut decoded).is_ok() {
						let mut rgb_frame = Video::empty();
						scaler.run(&decoded, &mut rgb_frame)?;

						// Converts a single frame into a vector of brightness
						// values between 128 and 255, into 50 rows of 78 u8's
						let mut pixels = rgb_frame.data(0)
							.chunks_exact(width_output as usize + 18)
							.map(|x| {
								let mut line = x[..x.len() - 18].iter()
									.map(|x| (*x as f64 * 0.5) as u8 + 128)
									.collect::<Vec<u8>>();
								
								// Add H-sync pulse to every line
								line.extend_from_slice(&vec![64u8;10]);

								line
							})
							.flatten()
							.collect::<Vec<u8>>();
						
						// Add V-sync pulse
						pixels.extend_from_slice(&vec![0u8;10]);
						
						for sample in pixels {
							let _ = writer.write_sample((sample as i16 - 128) * 256);
						}

						frame_index += 1;
					}

					Ok(())
				};

			for (stream, packet) in input_context.packets() {
				if stream.index() == video_stream_index {
					decoder.send_packet(&packet)?;
					receive_and_process_decoded_frames(&mut decoder)?;
				}
			}

			decoder.send_eof()?;
			receive_and_process_decoded_frames(&mut decoder)?;
		}

		let _ = writer.finalize();
	}

	Ok(())
}
