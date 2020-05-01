use crate::peer::Peer;

pub struct Interface<'a> {
    name: &'a str,
    public_key: [u8; 32],
    private_key: [u8; 32],
    listening_port: u16,
    peers: Vec<Peer<'a>>,
}

impl<'a> Interface<'a> {
    pub fn new(
        name: &'a str,
        public_key: [u8; 32],
        private_key: [u8; 32],
        listening_port: u16,
        peers: Vec<Peer<'a>>,
    ) -> Self {
        Self {
            name,
            public_key,
            private_key,
            listening_port,
            peers,
        }
    }
}
