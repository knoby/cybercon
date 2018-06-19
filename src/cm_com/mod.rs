mod msg_gen;



pub fn samplemsg() {
	println!("{:?}", msg_gen::gen_msg(msg_gen::OpCodes::Alive,false));
}