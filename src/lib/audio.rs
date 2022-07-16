use hound::{SampleFormat, WavWriter};
use std::io::BufWriter;
use std::fs::File;

pub type Writer = WavWriter<BufWriter<File>>;

pub fn create_writer () -> Writer {
	let spec = hound::WavSpec {
		channels: 1,
		sample_rate: 44100,
		bits_per_sample: 16,
		sample_format: SampleFormat::Int,
	};

	WavWriter::create("output.wav", spec).unwrap()
}

pub fn buffer_to_wav (buffer : &[u8]) {
	let mut writer = create_writer();

	for sample in buffer {
		let real_sample : i16 = (*sample as i16 - 128) * 255;
		let _ = writer.write_sample(real_sample);
	}

	let _ = writer.finalize();
}
