
mod cm_com;


fn main() {
	cm_com::connect("/dev/ttyUSB0".to_string());
}