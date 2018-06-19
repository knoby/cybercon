
/// Operation Codes for the Cybermaster
pub enum OpCodes {
	Alive,
	PlaySound(u8),
	UnlockFirmware,
}


/// Generate the Message for the given OP Code and data
pub fn gen_msg(op: OpCodes, toggle: bool) -> Vec<u8> {
	//Generate the Payload
	let payload = gen_payload(op, toggle);
	let payload_iter = payload.iter();

	//Init message
	let mut msg: Vec<u8> = vec!(0xfe, 0x00, 0x00, 0xff);

	let mut check_sum: u8 = 0x00;

	//Iterate over the Payload and build message
	for val in payload_iter {
		//Append payload
		msg.push(*val);
		//Append Check for current payload
		msg.push(!*val);
		//Add payload to checksum
		check_sum = check_sum.wrapping_add(*val);
	}
	//Append Checksum
	msg.push(check_sum);
	//Append Checksum Check
	msg.push(!check_sum);

	//Return message
	msg
}


/// Check the generated OP Code and Data for the Functions
fn gen_payload(op: OpCodes, toggle: bool) -> Vec<u8> {

	let code: u8;

	if toggle {
	    code = 0x08;
	} else {
		code = 0x00;
	}

	match op {
		OpCodes::Alive => vec!(code | 0x10),
		OpCodes::PlaySound(tone) => vec!(code | 0x51, tone),
		OpCodes::UnlockFirmware => vec!(code | 0xa5, 0x44, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x62, 0x79, 0x74, 0x65, 0x2c, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x49, 0x20, 0x6b, 0x6e, 0x6f, 0x63, 0x6b, 0x3f),
	}


}


#[cfg(test)]
mod tests {
	#[test]
	fn payload_check() {
		use cm_com::msg_gen::*;

		print!("Checking Alive...");
		assert_eq!(gen_payload(OpCodes::Alive, false), vec!(0x10));
		assert_eq!(gen_payload(OpCodes::Alive, true), vec!(0x18));
		println!("Ok");

		print!("Checking PlaySound...");
		assert_eq!(gen_payload(OpCodes::PlaySound(1), false), vec!(0x51, 0x01));
		assert_eq!(gen_payload(OpCodes::PlaySound(2), true), vec!(0x59, 0x02));
		println!("Ok");

		print!("Checking UnlockFirmware...");
		assert_eq!(gen_payload(OpCodes::UnlockFirmware, false), 
			vec![0xa5, 0x44, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x62, 0x79, 0x74, 0x65, 0x2c, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x49, 0x20, 0x6b, 0x6e, 0x6f, 0x63, 0x6b, 0x3f]);
		println!("Ok");
	}

	#[test]
	fn msg_check() {
		use cm_com::msg_gen::*;

		print!("Testing Alive MSG...");
		assert_eq!(gen_msg(OpCodes::Alive, false), vec![0xfe, 0x00, 0x00, 0xff, 0x10, 0xef, 0x10, 0xef ]);
		println!("Ok");

		print!("Testing UnlockFirmware MSG...");
		assert_eq!(gen_msg(OpCodes::UnlockFirmware, false), vec![0xfe, 0x00, 0x00, 0xff, 0xa5, 
			0x5a, 0x44, 0xbb, 0x6f, 0x90, 
			0x20, 0xdf, 0x79, 0x86, 0x6f, 
			0x90, 0x75, 0x8a, 0x20, 0xdf, 
			0x62, 0x9d, 0x79, 0x86, 0x74, 
			0x8b, 0x65, 0x9a, 0x2c, 0xd3, 
			0x20, 0xdf, 0x77, 0x88, 0x68, 
			0x97, 0x65, 0x9a, 0x6e, 0x91, 
			0x20, 0xdf, 0x49, 0xb6, 0x20, 
			0xdf, 0x6b, 0x94, 0x6e, 0x91, 
			0x6f, 0x90, 0x63, 0x9c, 0x6b, 
			0x94, 0x3f, 0xc0, 0x85, 0x7a]);
		println!("Ok");

		print!("Testing PlaySound MSG...");
		assert_eq!(gen_msg(OpCodes::PlaySound(5), false), vec![0xfe, 0x00, 0x00, 0xff, 0x51, 0xae, 0x05, 0xfa, 0x56, 0xa9 ]);
		println!("Ok");

	}
}