use crate::ip;

pub fn ip_class(ip: &ip::Ip) -> String {
	if ip.first <= 127 {
		"A".to_string()
	} else if 128 <= ip.first && ip.first <= 191 {
		"B".to_string()
	} else if 192 <= ip.first && ip.first <= 223 {
		"C".to_string()
	} else {
		"?".to_string()
	}
}