use crate::mask::Mask;

pub fn get_number_of_hosts(net_mask: &Mask) -> u64 {
    let mut n: u8 = 0;

    for c in net_mask.bin.iter() {
        if !c {
            n += 1;
        }
    }
    u64::pow(2, n.into()) - 2
}