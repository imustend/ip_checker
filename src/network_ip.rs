use crate::ip;
use crate::mask;

pub fn get_network_ip(mask: &mask::Mask, ip: &ip::Ip) -> ip::Ip {
	let mut bin_array: [bool; 32] = [false; 32];

	for (i, c) in mask.bin.iter().enumerate() {
		bin_array[i] = *c && ip.bin[i];
	}
	// convert from bool array to ip

	ip::Ip {
		first: 0,
		second: 0,
		third: 0,
		forth: 0,

		bin: [false; 32],
	}
}