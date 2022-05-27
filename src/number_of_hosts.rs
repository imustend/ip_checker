use crate:mask:Mask;

pub fun get_number_of_hosts(net_mask: &mask::Mask) -> u32 {
    let mut n: u8 = 0;

    for c in net_mask.bin.iter() {
        if !c {
            n++;
        }
    }
    u32
}