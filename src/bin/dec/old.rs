use cpal::traits::{DeviceTrait, HostTrait, EventLoopTrait};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use lib::env::{init, env};
// use hound;
use lib::audio::create_writer;

fn convert_sample (sample : u16) -> i16 {
	// (sample as i32 - 32768).abs() as u16
	(sample as i32 - 32768) as i16
}

fn main () {
	init();

	let host = cpal::default_host();

	let device = host
		.default_input_device()
		.expect("no input device available");

	let format = device
		.default_input_format()
		.expect("Failed to get default input format");

	let recording_length = env("RECORDING_LENGTH").parse::<u64>().unwrap();

	let event_loop = host.event_loop();
	let stream_id = event_loop
		.build_input_stream(&device, &format)
		.expect("Could not build input stream");


	event_loop.play_stream(stream_id).expect("Could not play stream");

	let recording = Arc::new(AtomicBool::new(true));
	let recording_event = recording.clone();
	let writer = RwLock::new(create_writer());

	thread::spawn(move || {
		event_loop.run(move |id, event| {

			let mut writer = writer.write().unwrap();

			// Stop the loop if recording should stop
			if recording_event.load(Ordering::Relaxed) == false {
				return;
			}

			let data = match event {
				Ok(data) => data,
				Err(err) => {
					eprintln!("An error occurred on stream {:?}: {}", id, err);
					return;
				}
			};

			match data {
				cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::U16(buffer) } => {
					for sample in buffer.iter() {
						let _ = writer.write_sample(
							convert_sample(cpal::Sample::to_u16(sample))
						);
					}
				},
				cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::I16(buffer) } => {
					for sample in buffer.iter() {
						let _ = writer.write_sample(
							convert_sample(cpal::Sample::to_u16(sample))
						);
					}
				},
				cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::F32(buffer) } => {
					for sample in buffer.iter() {
						let _ = writer.write_sample(
							convert_sample(cpal::Sample::to_u16(sample))
						);
					}
				},
				_ => (),
			}
		});
	});

	// Record for 2 seconds
	thread::sleep(Duration::from_secs(recording_length));
	recording.store(false, Ordering::Relaxed);
}
