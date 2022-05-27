use pad::{PadStr, Alignment};

pub struct Mask {
	pub first: u8,
    pub second: u8,
    pub third: u8,
    pub forth: u8,

    pub bin: [bool; 32]
}

pub fn build_mask(ip: String) -> Mask {
    let split = ip.replace(".", ":");
    let split = split.split(":");
    let split_vec = split.collect::<Vec<&str>>();

	// let split_vec = split_vec;
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
	


	// validating the mask
	let mut changed: bool = false;
	for c in bin.iter() {
		if *c && changed {
			panic!("invalide mask!")
		}

		changed = !*c;
	}
	

    Mask {
        first: octets[0],
        second: octets[1],
        third: octets[2],
        forth: octets[3], 
        bin: bin
    }
}