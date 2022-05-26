use crate::ip;
use crate::mask;

pub fn print_ip(name: String, ip: &ip::Ip) {
	println!("ip {}: ", name);
	println!("{}.{}.{}.{}", ip.first, ip.second, ip.third, ip.forth);
	for c in ip.bin.iter() {
		print!("{}", *c as i32);
	}
	println!("");
	println!("");
}

pub fn print_musk(name: String, ip: &mask::Mask) {
	println!("mask {}: ", name);
	println!("{}.{}.{}.{}", ip.first, ip.second, ip.third, ip.forth);
	for c in ip.bin.iter() {
		print!("{}", *c as i32);
	}
	println!("");
	println!("");
}