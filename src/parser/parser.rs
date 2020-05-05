use crate::interface::Interface;
use std::collections::BTreeMap;
use std::convert::TryInto;

use std::path::Path;

pub fn parse(path: &Path) -> Result<Interface, String> {
    if let Ok(contents) = &std::fs::read_to_string(path) {

        let interface: BTreeMap<String, String> = serde_yaml::from_str(contents).unwrap();

        return Ok(Interface::new(
            "todo",
            "000000000000000000000000000000000000000000000000000000000000000"
                .as_bytes()
                .try_into()
                .expect("slice with incorrect length"),
            interface
                .get("PrivateKey")
                .unwrap()
                .as_bytes()
                .try_into()
                .expect("slice with incorrect length"),
            interface.get("ListenPort").unwrap().parse::<u16>().unwrap(),
            vec![],
        ));
    }

    Err(String::from("An error occured while parsing the interface"))
}
