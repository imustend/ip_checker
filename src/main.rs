use broadcast_ip::get_broadcast_ip;

pub mod mask;
pub mod ip;
pub mod show;
pub mod ip_class;
pub mod network_ip;
pub mod number_of_hosts;
pub mod broadcast_ip;

fn main() {
    let ip = ip::build_ip(" 	172.16.0.0 ".to_string());
	let mask = mask::build_mask("255.224.0.0".to_string());

	let broadcast = broadcast_ip::get_broadcast_ip(&ip, &mask);

	show::print_ip("broadcast".to_string(), &broadcast);
}