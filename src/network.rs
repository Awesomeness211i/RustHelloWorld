enum Status {
	Ok,
	Disconnected,
	None,
}
pub enum Ipa {
	v4(u8, u8, u8, u8),
	v6(u16, u16, u16, u16, u16, u16, u16, u16)
}
struct Connection {
	status: Status,
	ipa: Ipa,
	port: u16
}

impl Connection {
	pub fn new(ipa: Ipa, port: u16) -> Connection {
		return Connection { status: Status::Ok, ipa, port };
	}
}

impl Ipa {
	pub fn to_string(&self) -> String {
		match self {
			Ipa::v4(i0, i1, i2, i3) => { return std::format!("{}.{}.{}.{}", i0, i1, i2, i3); }
			Ipa::v6(i0, i1, i2, i3, i4, i5, i6, i7) => { return std::format!("{}:{}:{}:{}:{}:{}:{}:{}", i0, i1, i2, i3, i4, i5, i6, i7); }
		}
	}
}