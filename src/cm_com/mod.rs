extern crate serial;

mod msg_gen;

use cm_com::serial::prelude::*;
use std::io::prelude::*;
use self::msg_gen::OpCodes;

use std::{thread, time};


pub fn connect(port: String) ->  Result<(), serial::Error> {

	//Try to open connection to serial port
	let sp = serial::open(&port);

	let mut sp = match sp {
		Ok(serial_port) => serial_port,
		Err(e) => return Err(e),
	};

	//Connection opend successfull --> reconfigure port
	sp.reconfigure(&|settings| {
		settings.set_baud_rate(serial::Baud2400)?;
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityOdd);
		settings.set_stop_bits(serial::Stop1);
		settings.set_flow_control(serial::FlowNone);
		Ok(())
	})?;
	//Set signal pins
	sp.set_dtr(true)?;
	sp.set_rts(false)?;


	//Connection Test
	let one_second = time::Duration::from_millis(1000);
	let mut serial_buf: Vec<u8> = vec![0; 100];	

	sp.write_all(&msg_gen::gen_msg(OpCodes::Alive, false))?;
	thread::sleep(one_second);
	match sp.read(serial_buf.as_mut_slice()) {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("{:?}", e),
    }
	println!("{:?}", &serial_buf);
	sp.write_all(&msg_gen::gen_msg(OpCodes::UnlockFirmware, false))?;
	thread::sleep(one_second);
	match sp.read(serial_buf.as_mut_slice()) {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("{:?}", e),
    }
	println!("{:?}", &serial_buf);
	sp.write_all(&msg_gen::gen_msg(OpCodes::PlaySound(0), false))?;
	thread::sleep(one_second);
	match sp.read(serial_buf.as_mut_slice()) {
        Ok(t) => println!("{:?}", t),
        Err(e) => eprintln!("{:?}", e),
    }
	println!("{:?}", &serial_buf);

	Ok(())
}
