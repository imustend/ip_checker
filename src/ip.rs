use pad::{PadStr, Alignment};

pub struct Ip {
	pub first: u8,
    pub second: u8,
    pub third: u8,
    pub forth: u8,

    pub bin: [bool; 32]
}



pub fn build_ip(ip: String) -> Ip {
    let split = ip.replace(".", ":");
    let split = split.split(":");
    let split_vec = split.collect::<Vec<&str>>();
    let mut bin: [bool; 32] = [false; 32];
	let mut octets: [u8; 4] = [0; 4];

    if split_vec.len() != 4 {
        panic!("Wrong amount of octets!");
    }
	for i in 0..4 {
		let octet: u8 = match split_vec[i].trim().parse() {
			Ok(num) => num,
			Err(_) => panic!("Something wrong with octet number {}", i + 1),
		};
		octets[i] = octet;

		let soctet = format!("{:b}", octet).pad(8, '0', Alignment::Right, true);




		for (j, c) in soctet.chars().enumerate() {
			bin[j + i*8] = c == '1';
		}
	}


    Ip {
        first: octets[0],
        second: octets[1],
        third: octets[2],
        forth: octets[3], 
        bin: bin
    }
}