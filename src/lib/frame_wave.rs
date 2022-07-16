pub fn convert_row (row : &[u8]) -> Vec<u8> {
	let mut output = row.to_owned();

	output.append(&mut vec![32u8; 10]);

	output
}

pub fn convert_frame (frame : &[Vec<u8>]) -> Vec<u8> {
	let mut output : Vec<Vec<u8>> = vec![];

	for line in frame {
		output.push(convert_row(&line));
	}

	let mut output : Vec<u8> = output.into_iter().flatten().collect::<Vec<u8>>();

	output.append(&mut vec![0u8; 10]);

	output
}
