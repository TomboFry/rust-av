use std::time::{Instant, Duration};
use std::thread::sleep;

pub fn throttle (start : Instant, max_duration_ms : u64) {
	let elapsed = start.elapsed().as_millis() as u64;
	if elapsed < max_duration_ms {
		let time_to_sleep = max_duration_ms - elapsed;
		println!("Sleeping for {}ms", time_to_sleep);
		sleep(Duration::from_millis(time_to_sleep));
	}
}

pub fn number_to_digits (number : u64, length : u64) -> String {
	if length < 1 { return format!("{}", number); }

	let mut output = String::new();

	for i in 0..length {
		if number < 10u64.pow(i as u32) {
			output = [ "0", &output ].concat();
		}
	}

	format!("{}{}", output, number)
}
