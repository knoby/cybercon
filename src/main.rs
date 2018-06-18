extern crate serialport;

use serialport::prelude::*;
use std::{thread, time};

use std::io::{self, Write};

use std::time::Duration;



fn main() -> std::io::Result<()> {
   

    let alive: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0x10, 0xef, 0x10, 0xef ];
    let unlock: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0xa5, 0x5a, 0x44, 0xbb, 0x6f, 0x90, 0x20, 0xdf, 0x79, 0x86, 0x6f, 0x90, 0x75, 0x8a, 0x20, 0xdf, 0x62, 0x9d, 0x79, 0x86, 0x74, 0x8b, 0x65, 0x9a, 0x2c, 0xd3, 0x20, 0xdf, 0x77, 0x88, 0x68, 0x97, 0x65, 0x9a, 0x6e, 0x91, 0x20, 0xdf, 0x49, 0xb6, 0x20, 0xdf, 0x6b, 0x94, 0x6e, 0x91, 0x6f, 0x90, 0x63, 0x9c, 0x6b, 0x94, 0x3f, 0xc0, 0x85, 0x7a];
    let kill1: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0x50, 0xaf, 0x50, 0xaf ];
	let kill2: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0x40, 0xbf, 0x40, 0xbf  ];
	let kill3: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0x70, 0x8f, 0x70, 0x8f ];
    let tone: Vec<u8> = vec![0xfe, 0x00, 0x00, 0xff, 0x51, 0xae, 0x05, 0xfa, 0x56, 0xa9 ];

	let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(2000);
    settings.baud_rate = serialport::BaudRate::Baud2400;
    settings.parity = serialport::Parity::Odd;
    settings.stop_bits = serialport::StopBits::One;
    settings.data_bits = serialport::DataBits::Eight;
    settings.flow_control = serialport::FlowControl::None;

	let port_name = "/dev/ttyUSB0".to_string();
   	

    if let Ok(mut port) = serialport::open_with_settings(&port_name, &settings) {
        let mut serial_buf: Vec<u8> = vec![0; 512];		
        port.write_data_terminal_ready(true)?;
        port.write_request_to_send(false)?;	
        loop {
        	port.write_all(&alive)?;
        	thread::sleep(time::Duration::from_millis(500));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            } 
        	port.write_all(&unlock)?;
        	thread::sleep(time::Duration::from_millis(1000));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            } 
        	port.write_all(&kill1)?;
        	thread::sleep(time::Duration::from_millis(500));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        	port.write_all(&kill2)?;
        	thread::sleep(time::Duration::from_millis(500));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
            port.write_all(&kill3)?;
            thread::sleep(time::Duration::from_millis(500));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
            port.write_all(&tone)?;
            thread::sleep(time::Duration::from_millis(500));
			match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => println!("{:?}", t),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    } else {
        println!("Error: Port '{}' not available", &port_name);
    }


    Ok(())
}