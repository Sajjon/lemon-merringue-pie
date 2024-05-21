use std::str::FromStr;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ApplianceID(Uuid);

uniffi::custom_type!(ApplianceID, Uuid, {
    from_custom: |appliance_id| appliance_id.0,
    try_into_custom: |uuid| Ok(ApplianceID(uuid))
});

#[uniffi::export]
pub fn new_appliance_id_random() -> ApplianceID {
    ApplianceID(Uuid::new_v4())
}

#[uniffi::export]
pub fn new_appliance_id_default() -> ApplianceID {
    ApplianceID::default()
}

uniffi::custom_type!(Uuid, String, {
    remote,
    from_custom: |id| id.to_string(),
    try_into_custom: |s| Ok(Uuid::from_str(&s)?)
});

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct BagOfBytes {
    pub bytes: Vec<u8>,
}

#[uniffi::export]
pub fn bag_of_bytes_to_hex_string(bag_of_bytes: BagOfBytes) -> String {
    hex::encode(bag_of_bytes.bytes)
}

/// # Panics
/// Panics if hex is not a valid hex string
#[uniffi::export]
pub fn new_bag_of_bytes_from_hex_string(hex: String) -> BagOfBytes {
    BagOfBytes {
        bytes: hex::decode(hex).unwrap(),
    }
}

#[uniffi::export]
pub fn new_bag_of_bytes_prepend_de(bag_of_bytes: BagOfBytes) -> BagOfBytes {
    let mut vec = bag_of_bytes.bytes;
    vec.insert(0, 0xde);
    BagOfBytes { bytes: vec }
}

#[uniffi::export]
pub fn new_bag_of_bytes_append_ef(bag_of_bytes: BagOfBytes) -> BagOfBytes {
    let mut vec = bag_of_bytes.bytes;
    vec.push(0xef);
    BagOfBytes { bytes: vec }
}

impl BagOfBytes {
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a clone of the inner bytes as a `Vec`.
    pub fn to_vec(&self) -> Vec<u8> {
        Vec::from(self.bytes())
    }

    /// Returns a references to the inner array slice.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

fn twos_complement_of_u8(u: u8) -> i8 {
    // Yes, it is this easy, Rust does all the heavy lifting
    u as i8
}

fn twos_complement_of_i8(i: i8) -> u8 {
    // Yes, it is this easy, Rust does all the heavy lifting
    i as u8
}

impl From<Vec<u8>> for BagOfBytes {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}

impl From<&[u8]> for BagOfBytes {
    /// Instantiates a new `BagOfBytes` from the bytes.
    fn from(value: &[u8]) -> Self {
        Self {
            bytes: value.to_vec(),
        }
    }
}

/*
 Expose `BagOfBytes` to Uniffi as `sequence<i8>`, unfortunately we cannot
 use `sequence<u8>` because it results in:

 /uniffi-rs-6f89edd2a1ffa4bd/fb8dd5c/uniffi_bindgen/src/interface/universe.rs:50:17:
 assertion `left == right` failed
 left: Custom { module_path: "profile", name: "BagOfBytes", builtin: Bytes }
 right: Custom { module_path: "profile", name: "BagOfBytes", builtin: Sequence { inner_type: UInt8 } }

 So HACK HACK HACK we use `sequence<i8>` (`Vec<i8>`) instead as an intermediary `Builtin`.

 However, in `uniffi.toml` we provide `from_custom`` / `into_custom`` for Kotlin and Swift
 which using two's complement maps back Vec<i8> -> Vec<u8>, meaning Kotlin and Swift actually
 never see the `i8`, and only works with u8.

 So we translate:
 Kotlin: `Rust[BagOfBytes <:2's comp.:> Vec<i8>] <:2's comp:> [Kotlin]List<UByte>`
 Swift:  `Rust[BagOfBytes <:2's comp.:> Vec<i8>] <:2's comp:> [Swift]Foundation.Data`

*/
uniffi::custom_type!(BagOfBytes, Vec<i8>, {
    from_custom: |bag| bag.to_vec()
    .into_iter()
    .map(twos_complement_of_u8)
    .collect_vec(),
    try_into_custom: |bytes|   Ok(bytes
        .into_iter()
        .map(twos_complement_of_i8)
        .collect_vec()
        .into()),
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_of_bytes() {
        let mut x = new_bag_of_bytes_from_hex_string("adbe".to_owned());
        x = new_bag_of_bytes_append_ef(x);
        x = new_bag_of_bytes_prepend_de(x);
        assert_eq!(bag_of_bytes_to_hex_string(x), "deadbeef".to_owned());
    }
}
