pub struct Peer<'a> {
    name: Option<&'a str>,
    public_key: [u8; 32],
    allowed_ips: &'a str,
}

impl<'a> Peer<'a> {
    pub fn new_named(name: &'a str, public_key: [u8; 32], allowed_ips: &'a str) -> Self {
        Self {
            name: Some(name),
            public_key,
            allowed_ips,
        }
    }

    pub fn new(public_key: [u8; 32], allowed_ips: &'a str) -> Self {
        Self {
            name: None,
            public_key,
            allowed_ips,
        }
    }
}
