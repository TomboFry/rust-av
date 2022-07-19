pub fn convert_row (row : &[u8]) -> Vec<u8> {
	let mut output = row.to_owned();

	output.extend_from_slice(&vec![64u8; 10]);

	output
}

pub fn convert_frame (frame : &[Vec<u8>]) -> Vec<u8> {
	let mut output : Vec<u8> = vec![];

	for line in frame {
		output.extend_from_slice(&convert_row(&line));
	}

	output.extend_from_slice(&vec![0u8; 10]);

	output
}
