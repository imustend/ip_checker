pub mod mask;
pub mod ip;
pub mod show;
pub mod ip_class;
pub mod network_ip;
pub mod number_of_hosts;
pub mod broadcast_ip;

fn main() {
    let ip = ip::build_ip("172.16.0.0 ".to_string());
	show::print_ip("ip".to_string(), &ip);
	println!("ip class: {}", ip_class::ip_class(&ip));

	let mask = mask::build_mask("255.224.0.0".to_string());
	show::print_musk("mask".to_string(), &mask);
	println!("number of hosts: {}", number_of_hosts::get_number_of_hosts(&mask));

	show::print_ip("network ip".to_string(), &network_ip::get_network_ip(&mask, &ip));

	let broadcast = broadcast_ip::get_broadcast_ip(&ip, &mask);
	show::print_ip("broadcast ip".to_string(), &broadcast);

}