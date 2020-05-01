use crate::interface::Interface;
use std::convert::TryInto;

use std::path::Path;

pub fn parse(path: &Path) -> Interface {

    Interface::new(
        "wgx",
        "000000000000000000000000000000000000000000000000000000000000000".as_bytes().try_into().expect("slice with incorrect length"),
        "000000000000000000000000000000000000000000000000000000000000000".as_bytes().try_into().expect("slice with incorrect length"),
        51820,
        vec![]
    )
}
