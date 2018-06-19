extern crate serial;

mod msg_gen;

use cm_com::serial::prelude::*;

fn connect_cm(port: &String) ->  Result<(), serial::Error> {

	//Try to open connection to serial port
	let sp = serial::open(port);

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

	Ok(())
}
