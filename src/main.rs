pub mod mask;
pub mod ip;
pub mod show;
pub mod ip_class;
pub mod network_ip;
pub mod number_of_hosts;
pub mod broadcast_ip;

use std::io;

fn main() {
	let mut ip_string = String::new();
	let mut mask_string = String::new();

	println!("ip?");
    io::stdin()
        .read_line(&mut ip_string)
        .expect("Failed to read line");

	println!("mask?");
    io::stdin()
        .read_line(&mut mask_string)
        .expect("Failed to read line");

    let ip = ip::build_ip(ip_string);
	show::print_ip("ip".to_string(), &ip);
	println!("ip class: {}", ip_class::ip_class(&ip));

	let mask = mask::build_mask(mask_string);
	show::print_musk("mask".to_string(), &mask);
	println!("number of hosts: {}", number_of_hosts::get_number_of_hosts(&mask));

	show::print_ip("network ip".to_string(), &network_ip::get_network_ip(&mask, &ip));

	let broadcast = broadcast_ip::get_broadcast_ip(&ip, &mask);
	show::print_ip("broadcast ip".to_string(), &broadcast);

}