use crate::mask;
use crate::ip;

pub fn get_broadcast_ip(net_ip: &ip::Ip, net_mask: &mask::Mask) -> ip::Ip {
	let mut bin_array: [bool; 32] = [false; 32];

	for (i, c) in net_mask.bin.iter().enumerate() {
		bin_array[i] = !*c || net_ip.bin[i];
	}

	// convert from bool array to ip
	let mut octets: [u8; 4] = [0; 4];
	
	let mut octet_bin: String;

	for i in 0..4 {
		octet_bin = "".to_string();
		for j in 0..8 {
			octet_bin += {
				if bin_array[j + i*8] {
					"1"
				} else {
					"0"
				}
			}
		}
		octets[i] = match u8::from_str_radix(&octet_bin, 2){
			Ok(num) => num,
			Err(_) => panic!("???"),
		};
	}


	ip::Ip {
		first: octets[0],
		second: octets[1],
		third: octets[2],
		forth: octets[3],


		bin: bin_array,
	}
}