use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigKeyCombination {
    pub is_alt: bool,
    pub is_ctrl: bool,
    pub is_shift: bool,
    pub keyname: String,
    pub value: String,
}

impl<'de> Deserialize<'de> for ConfigKeyCombination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: String = Deserialize::deserialize(deserializer)?;
        let keys: Vec<&str> = val.split(".").collect();
        let key_iter = keys.iter();
        let is_alt = key_iter.clone().any(|&x| x == "leftalt" || x == "rightalt");
        let is_ctrl = key_iter
            .clone()
            .any(|&x| x == "leftctrl" || x == "capslock");
        let is_shift = key_iter.clone().any(|&x| x == "leftshift");
        let keyname = keys.last().expect("Failed to get key value");
        Ok(ConfigKeyCombination {
            is_alt: is_alt,
            is_ctrl: is_ctrl,
            is_shift: is_shift,
            keyname: keyname.to_string(),
            value: val,
        })
    }
}
