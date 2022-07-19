use hound::{SampleFormat, WavWriter};
use std::io::BufWriter;
use std::fs::File;
use std::path::Path;

pub type Writer = WavWriter<BufWriter<File>>;

/// Create a WAV file at 44.1kHz, 16-bit
pub fn create_writer(path: &Path) -> Writer {
	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: 44100,
		bits_per_sample: 16,
		sample_format: SampleFormat::Int,
	};

	let path = path.file_stem().unwrap().to_str().unwrap();
	WavWriter::create(format!("{}.wav", path), spec).unwrap()
}

/// Convert a long list of 8-bit integers to 16-bit WAV file
pub fn buffer_to_wav(buffer: &[u8], path: &Path) {
	let mut writer = create_writer(path);

	for sample in buffer {
		let real_sample: i16 = (*sample as i16 - 128) * 255;
		let _ = writer.write_sample(real_sample);
	}

	let _ = writer.finalize();
}
