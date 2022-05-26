pub mod mask;
pub mod ip;
pub mod show;
pub mod ip_class;
pub mod network_ip;

fn main() {
    let ip = ip::build_ip("124:225:25:83".to_string());
	let mask = mask::build_mask("255.255.255.252".to_string());

	show::print_ip("ip".to_string(), &ip);
	println!("{}", ip_class::ip_class(&ip));

	let ip_of_network = network_ip::get_network_ip(&mask, &ip);
	show::print_ip("networks ip".to_string(), &ip_of_network);
}